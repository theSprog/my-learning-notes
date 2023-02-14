### 异步引入

不通过多线程来达到多线程的效果



现代 CPU 的算力十分充沛但 IO 速度缓慢，此时如果采用同步方法调用 IO，在执行 IO 密集型任务时大部分的时间都花在 CPU 等待 IO 上，我们希望将 CPU 充分利用起来，在 IO 准备时也去执行许多与上下文不存在依赖关系的操作，例如同时发起多个 IO 请求，而不是每当一个 IO 完成后才发起下一个。



多线程固然可以完成这个任务，但多线程开销比较大，使用时才可能遇到数据竞争造成错误。而且多线程并非唯一的运行异步任务的手段，`Future` 搭配 `async/.await` 也是其中一种手段。

`Future` 是一种协作式多任务调度，而非类似于线程的抢占式多任务，这也就说明了除非某一个 `Future` 主动放弃CPU，否则它就会一直执行直到结束，这也说明如果使用不当可能会导致某些任务 "饿死"。

在 Rust 中 `Future` 是惰性的，不会一创建就可以执行，而是直到我们手动调用它时才会执行



### Future 内部

```rust
pub trait Future {
    type Output;
	fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<self:Output>;
}

pub enum Poll<T>{
    Ready(T),
    Pending,
}
```

`Future` 包括一个关联类型 `Output`，它是指该 `Future` 返回的数据类型。

`poll()` 方法返回一个 `Poll`，它是一个枚举，内部包含两种状态：

- 如果任务已经完成，则返回 `Ready` 状态，内部包含一个 `T` 类型数据（根据上下文可知也就是 `Output` 类型）
- 如果任务还没有完成而且正在阻塞，则返回 `Pending` 状态



#### Context结构体

```rust
pub struct Context<'a> {
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

Rust 自身并不提供异步运行时，它只在标准库里规定了一些基本的接口，至于怎么实现，可以由各个运行时（如tokio)自行决定。所以在标准库中，你只会看到这些接口的定义，具体实现根据运行时不同而不同。



#### executor

由于有多个 `Future` 需要管理，因此我们需要一个调度器，用于调度某些可以继续执行的 `Future` ，这在 Rust 中就是 executor。Rust 在语言层面上不提供 executor，这就使得不需要 `Future` 的代码可以完全脱离内置 executor 带来的性能影响，而需要 `Future` 时可以自主选择生态圈中适合的 executor

当任务执行到某个地方需要被阻塞时，该任务就被 executor 挂起并设置好唤醒条件，让 reactor 去监听这些条件



#### reactor

当返回 `Pending` 时任务正在被挂起，我们需要某种机制监听唤醒该任务的条件，一旦该条件到来，就立马通知 executor 将某个任务加入就绪队列等待调度，完成这项功能的就是 reactor



### 异步处理逻辑

executor 会运行在多个线程上，运行线程自己的就绪队列上的 `Future`，如果没有，就去别的线程的调度器上"偷”一些过来运行（工作窃取）。

当某个任务无法再继续取得进展，此时 `Future` 返回的结果是 `Pending`，那么调度器会挂起任务，并设置好合适的唤醒条件以便让 reactor 监听。
而 reactor 会利用操作系统提供的异步l/O,比如 epoll/kqueue/IOCP,来监听操作系统提供的 IO 事件，当遇到满足条件的事件时，就会通过 `Waker.wake()` 通知 executor 将这个 Future 加入就绪队列等待调度。



举例来说

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let f1 = fs::read_to_string("./Cargo.toml");
    let f2 = fs::read_to_string("./Cargo.lock");
    try_join!(f1, f2)?;	// 启动这两个方法，异步运行
    let ns = start.elapsed().as_nanos();
    println!("async {:?} ns", ns);

    let start = Instant::now();
    stdfs::read_to_string("./Cargo.toml")?;	// 同步运行，只有该函数返回后才会执行下一条指令
    stdfs::read_to_string("./Cargo.lock")?;	// 同步运行
    let ns = start.elapsed().as_nanos();
    println!("sync {:?} ns", ns);
    Ok(())
}
```

上面的运行第一种方式是异步运行，当某一个 IO 阻塞时，Future 会自动让出CPU运行另外的 Future，直到所有的 Future  运行完成。

而第二种方式是同步运行，哪怕读取时被阻塞也不会继续运行下一个方法，读文件时我们不得不等待，直到整个文件就绪才能继续IO读取。

可以想见，第二种方式等待时间是 t1 + t2，而第一种等待时间是 max(t1, t2)。（严格来说，第一种应该比 max(t1, t2) 要多一些，因为有可能两个 IO 请求发起后都在等待，此时整个线程就被阻塞了，必须等待两个 IO 请求之一完成）

一般而言，越是 IO 密集型操作使用异步带来的操作收益越高



