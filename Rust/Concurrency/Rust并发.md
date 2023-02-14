### 语法

Rust 中使用标准库中的 `thread` 包操作线程



#### 创建线程

`thread` 包下的 `spawn(f)` 可以接受一个函数并且为其创建一个线程并执行它，它的返回值是一个 `JoinHandle<T>`，内部的 T 代表该线程的返回值类型。

`spawn()` 一旦调用线程就会立马进入就绪态，等待处理器调度，而没有类似 `Java` 中的 `.start()` 启动一个线程的机制。Rust 中的线程模型是 `1:1` 模型，也就是说一旦创建一个线程，在操作系统中就对应一个线程，它可以被放在 CPU 上调度。

Rust 中主线程一旦结束，则其他线程也默认退出。

```rust
fn main() {
    let thr = thread::spawn(|| {
        thread::sleep(Duration::from_millis(3000));	// Duration in std::time
        println!("I am wake up");
        42
    });
    let v: Result<T, Box<dyn Any + Send + 'static>> = thr.join();
    println!("{:?}", thr);	// nothing to print
}
```

如果要等待某个线程执行完成则可以对该线程使用 `join()`，该方法返回一个 `Result<T>`，T 就是线程返回值的类型。

`join()` 会转移所有权，因此某个线程不能够多次 `join()`，`join()` 之后也不能够再用于其他的操作，因为它的所有权已经被转移。

```rust
let v: Result<T, Box<dyn Any + Send>> = thr.join();
println!("{:?}", v);	// -> Ok(42)
```

主线程与子线程：Rust允许在线程中再创建线程，但是新创建的线程的父线程却仍然是主线程，而不是创建线程的那个线程

```rust
fn main() {
    let handle = thread::spawn(|| {		// 线程 1
        let thr = thread::current();
        thread::sleep(Duration::from_secs(1));

        // 注意，这个线程的父线程是主线程，而不是线程 1
        let handle2 = thread::spawn(|| {
            let thr2 = thread::current();
            thread::sleep(Duration::from_secs(1));
            return format!("{:?}: {:?}", thr2.name(), thr2.id());
        });

        return format!("{:?}: {:?}", thr.name(), thr.id());
    });
    let res = handle.join();
}
```





#### 传递参数

传递参数只能通过 `move` 的方式，因为父线程和子线程拥有不同的生命周期，如果使用引用的方式有可能出现父线程变量已经消亡但子线程却仍然持有该变量的引用，由此引发悬垂指针的情况，因此必须让函数捕获该变量的所有权。

但 `move` 的方式前提是在线程中传递的参数必须实现 `Send` 和 `Sync` trait

```rust
fn main() {
    let i = String::from("thread-42");
    let thr = thread::spawn(move || {
        thread::sleep(Duration::from_millis(3000));
        println!("I am {} and I am wake up", i);
        42
    });
    let v: Result<i32, Box<dyn Any + Send>> = thr.join();
    println!("{:?}", v);
}
```





#### 获取线程信息

在线程中可以通过 `thread::current()` 获取到当前线程，它是一个 `Thread` 结构体。从它我们可以获得 `name()` 、`id()` 之类的信息。

主线程的 `name()` 是 `main`，线程 `id()`  为 1，其他线程默认没有名称（`None`），`id` 从2开始计数。

```rust
fn main() {
    let handle = thread::spawn(|| {
        let thr = thread::current();
        thread::sleep(Duration::from_secs(1));
        return format!("{:?}: {:?}", thr.name(), thr.id());
    });
    let res = handle.join();
    println!("{:?}", res.unwrap());		// None: ThreadId(2)
    println!(
        "{:?}: {:?}",
        thread::current().name(),
        thread::current().id()
    );									// Some("main"): ThreadId(1)
}
```







### 线程间共享

线程间共享数据需要使用 `Arc` 智能指针避免数据竞争

```rust
fn main() {
    let s = Arc::new(String::from("hello"));
    for i in 0..10 {
        let s_clone = s.clone();	// clone 只是增加引用计数
        thread::spawn(move || {
            println!("{}", s_clone);
        });
    }
}
```



如果多线程之间还需要改变这些数据，就必须使用 `Mutex`，否则线程不安全，编译不通过

```rust
let s = Arc::new(Mutex::new(format!("hello")));
for i in 0..12 {
    let s_clone = s.clone();	// 实质上只是增加引用计数
    thread::spawn(move || {	// 将 s_clone 移入
        let mut s2 = s_clone.lock().unwrap();	// 获取锁，此时 s2 就能当作 String 的可变引用使用
        s2.push_str(&format!(" {}", i));
    });
}

thread::sleep(Duration::from_secs(2));	// -> "hello 0 1 2 4 5 3 10 11 7 8 9 6" (某一次的结果)
println!("{:?}", s);
```

我们并没有手动释放锁，因为 `lock().unwrap()` 得到的是一个 `MutexGuard<T>`，当他的生命周期结束时会自动释放锁，而不用手工管理。

 如果另一个线程在持有这个锁时发生了恐慌，那么一旦这个锁被当前线程获取，这个调用将返回给当前线程一个错误。如果同一个线程在持有锁的同时再次获取该锁，那么发生 panic





### 基本通道（basic channel）

属于库 `std::sync::mpsc::channel`。它是一个先入先出模型（`FIFO`）可以有多个生产者，单个消费者（Multi-producer, single-consumer（mpsc））。

向一个已被关闭的通道无论是发送还是接收数据都会产生错误。当所有 `sender` 都被 `drop` 时通道会自动关闭，同样，当 `receiver` （`mpsc` 的通道默认只有一个 `receiver` ）被 `drop` 时通道也会关闭。

默认的通道是异步通道，拥有无限多容量的`buffer`



#### 基本用法

我们可以在 Rust 中定义通道，并且在线程间使用通道通信。`channel<T>()` 定义一个元素类型为 `T` 的通道，返回一个元组，分别代表发送者 `sender` 和接收者 `receiver` 。

发送者可以使用 `send()` 向通道发送元素，该操作可能会失败，因此返回值为 `Result<(), E>`，即若成功则返回无意义的单元类型 `()`，否则返回错误。

同样接收者 `recv()` 从通道中取数据，该操作同样也可能失败，所以结果也是一个 `Result<T, E>`，其中 `T` 就是取到的元素类型

```rust
fn main() {
    let (sender, receiver) = channel::<i32>();

    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(200));
            sender.send(i).unwrap(); // panic if send fail
        }
    });

    while let Ok(v) = receiver.recv() {
        println!("receiver recv {}", v);
    }
}
```

上面代码中 `sender` 被移入子线程中，当循环结束 `sender` 就自动回收掉，当一个 `channel()` 的 `sender` 消亡时该通道也会被关闭，而从被关闭的通道中读取数据会产生一个 `Err(RecvError)` 错误。



#### 多发送者

`sender` 实现了 `Clone trait`，对一个 `sender` 进行克隆会产生一个新的发送者，它也可以往这个通道发送数据，通道会记录自己有多少个 `sender`，只有当所有的 `sender` 都消亡时通道才会关闭

```rust
let (sender, receiver) = channel::<i32>();

for i in 0..2 {
    let s = sender.clone();	// clone sender
    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(200));
            s.send(i).unwrap(); // panic if send fail
        }
    });
}

{
    sender;	// drop sender !
}

while let Ok(v) = receiver.recv() {
    println!("receiver recv {}", v);
}
```

可以使用新作用域来间接销毁 `sender`，也可以使用另一种方式：`std::mem::drop(sender);` 手动明确 drop。

必须注意的是，`receiver` 是不允许 `clone` 的，也就是说基本通道只运行多发送者不允许多接收者。



#### 迭代接受

`receiver` 实现了迭代器操作，所以可以用于 `for` 循环，直到接收到 `None` 就会退出。由于能进入循环体的元素都必然是非 `None`，因此会自动 `unwrap`。

```rust
for r in receiver {	// r auto unwrap
	println!("r = {}", r);	
}
```



### 同步通道（sync channel）

默认的基本通道是异步通道，换句话说接受和发送没有标准顺序。标准库 `mpsc` 还提供了同步通道，需要指明 `bound` 界限，即有多少个缓冲区，注意 0 也是可以的，这时这个通道就会变成无缓存通道，当只有`sender`和`receiver`都准备好了后它们的通讯才会发生，否则就会阻塞（即要求实时同步）

```rust
let (sd, rv) = mpsc::sync_channel::<i32>(5);	// 设置 5 个缓存
```

同样的，`sender` 也支持 `clone()`，而且新生成的 `sender` 和旧有的 `sender` 共享发送区缓存，而不会自己另开缓存。所以说容量限制其实是和 `receiver` 绑定的

同时该通道还新增了 `try_send` 方法，该方法企图去向一个通道无阻塞地发送消息，这个方法与`send`不同，如果通道的缓冲区满了或者通道已关闭，则立即返回错误。同样的，该通道还有 `try_recv` 方法，也是企图无阻塞地拿到数据，一旦发生缓冲区空或者通道关闭，则返回错误。这两个方法一旦调用就会立即返回。

```rust
fn main() {
    let (sd, rv) = mpsc::sync_channel::<i32>(0);
    
    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let sendinfo = sd.try_send(1);	// 尝试发送
        if let Err(v) = sendinfo {		// 如果失败，打印失败信息
            println!("{}", v);
            continue;
        }
        break;	// 否则退出
    });

    thread::sleep(Duration::from_secs(5));
    if let Ok(v) = rv.recv() {
        println!("get {}", v);
    }
}
```



### 控制线程顺序

#### Barrier

`Barrier` 位于 `std::sync::Barrier`

有时候我们需要等所有线程都完成某一个操作之后再一起开始，但是线程之间的操作是无序的，因此需要引入 `barrier` 来进行阻塞，每当一个线程到达某个 `barrier` 点时，`barrier` 计数就会减 1，若减到 0 则会取消阻塞，让被阻塞的线程一起运行。

```rust
let bar = Arc::new(Barrier::new(30));	// 假设初始值为 30
let mut handles = vec![];
for i in 0..30 {
    let bar_clone = bar.clone();
    let handle = thread::spawn(move || {
        println!("{:?} start", thread::current().id());
        bar_clone.wait();	// 每到达一个线程，该barrier就会减 1，直到减到 0 才会取消阻塞
        println!("{:?} end", thread::current().id());
    });
    handles.push(handle);
}

for handle in handles {
    handle.join();
}
```

一般而言需要控制的线程和`barrier`是一样多的，若 `barrier` 较多则发生线程全部被阻塞时 `barrier`还未归零从而死锁。重点提一下 `barrier` 较少时，由于 `barrier` 是可重用的，当放过某一批线程之后再次调用 `wait()` 则会将 `barrier` 重新计数，从而可能发生死锁（注意是可能，因为可能下一批线程不多不少，刚好可以足以解开 `barrier` 的阻拦）



#### Condvar

`Condvar` 位于 `std::sync::Condvar`