## 内存模型

内存模型的意义在于对特殊内存的读写访问过程的一般抽象。

Java 内存模型的作用就是屏蔽操作系统和硬件的差异，已实现所有 Java 程序在各个平台下都能保持一致的可观测访问效果。具体而言就是规定所有**共享变量**的访问规则。

> 这里专指共享变量的访问规则，像局部变量由于是线程私有的，所以没有共享一说。



## 主内存与工作内存

Java 规定所有共享变量存放在主内存中，所有线程都可以访问主内存，但不是直接访问，而是通过自己私有的工作线程与主内存交互。

Java 程序的所有读写访问操作都是在自己的工作内存完成的，线程不能直接访问其他线程的工作线程。



## 内存交互

《规范》要求以下八种指令都必须是原子的：

- lock / unlock：

  > 都是作用于主内存，它将一个变量标识为某个线程独占/不独占

- read / load

  > read 作用于主内存，它将主内存的变量传递至工作内存中。
  >
  > load 作用于工作内存的副本中，用于接受从主内存传递而来的变量，将其放入自己工作内存的副本中

- use / assign

  > use 作用于工作内存中，它把工作内存中的变量传入执行引擎中
  >
  > assign 作用于工作内存，它把从执行引擎中的值传入工作内存中

- store / write 

  > store 作用于工作内存，将工作内存的值传入主内存中
  >
  > write 作用于主内存，接受从工作内存传入的值



### 传递变量

要将一个变量从主内存移入工作内存中，依次进行 read、load 操作；反之则进行 store、write 操作。《规范》规定两个指令（无论是读取还是写入过程）都必须顺序执行，但可以不用连续执行，也就是说可以 read 与 load 之间和 store 与 write 之间可以插入其他指令，但是不允许 read / load / store / write 操作单独出现。



### 规则

- 任何一个线程在进行 assign 后都要将这种变化同步回主内存，反之如果没有发生过 assign 则不允许同步回主内存中，这样做是为了防止已经被另一个线程更改后的变量又重新被本线程同步回原值。
- 同一个共享变量可以被同一线程多次 lock，但是也要 unlock 相同次数才能解锁
- 对一个变量进行 lock 后如果原来工作内存中有该变量，那么必须将此变量清零，以重新 read、load。
- 对一个变量 unlock 前必须将此变量同步回主内存中。
- 如果一个变量没有被 lock，那么不允许对它进行 unlock 操作。



### volatile

#### 作用

volatile 是 Java 提供的最简单的同步机制，他有两条作用：

- 当某个变量被定义为 volatile 后，它的所有修改对所有线程可见。

  > 这里的 "可见" 是指某一个线程修改它后必须立即将其同步回主内存中，以使得所有线程都看见它的变化
  >
  > 而其他普通变量不一定能够有这种性质，例如某一个变量本来是 5，被修改为 3 后，其他线程可能还是会看见他是 5
  >
  > 每一个都 volatile 变量的操作都要去主内存中进行 read、load 操作（即使没有修改也是如此），这就保证了该变量在所有线程中的一致性。
  >
  > 
  >
  > 注意：volatile 虽然是所有线程可见的，但这并不意味着它是线程安全点。举例来说：
  >
  > 假设变量 `a` 是 volatile 的，但是 `a++` 并不是原子操作，当 `a` 原本是 1 时它自增产生新副本 `temp` 值是 2，此时 `a` 被另一线程改为 2，但是写回还是将 temp(2) 写回变量，因此并不保证线程安全

- 禁止指令重排序优化

  > 普通变量只能保证在可观测点获取到正确结果，而中间的过程可以重排序以优化程序。
  >
  > 但是一旦一个变量设置为 `volatile` 时就不允许进行重排序。

#### 规则

当某一个变量被 volatile 修饰时它必须履行以下规则

- load 和 use 必须**连续**使用

  > 这条规则保证了每次对某一个 volatile 变量的使用都必须从主内存中取值，以保证是最新值

- assign 和 store 必须**连续**使用

  > 这代表每次对 volatile 的修改都必须立马同步回主内存中，一旦执行引擎写出工作内存都必须马上再写回主内存。

- 内存屏障（memory fence）之后的指令不能重排序到其之前



#### 可见性

除了 volatile 可以保证 可见性 之外，synchronized 和 final 也可以保证可见性

- synchronized 

  > `synchronized` 可以保证可见性，一个线程退出 `synchronized` 代码块后，他在该代码块内执行的所有结果都会被其他线程看见

- final

  > 被 final 修饰的字段，一旦在构造器中被初始化完成，那么其他线程就能看见 final 字段的值。这里有个难以理解的特例：this 字段没有逃逸，而如果它逃逸了，那么它就可能会被观测到只初始化一半。



## Happens-Before 原则

如果一个操作 happens-before 另一个操作，则前一个操作对于后一个操作是可见的。我们会看到哪怕有时候某一指令确实先于（指时间上）另一指令发生，但是它并不 happens-before 于这一指令，导致后发生的指令看不到之前那个指令的效果。



具体包括：

- 单线程原则：单个线程内如果 a 先于 b 运行，那么a happens-before 于 b。重排序的话以实际运行顺序为准
- 锁操作原则：对于同一把锁，拿到锁的线程能看到之前获得锁的线程的全部操作
- volatile原则：被 volatile 修饰的变量具有可见性
- 线程启动：子线程启动时能看到父线程之前发生的所有操作
- join原则：join之后的语句能够看到运行的线程完毕时的所有操作
- 传递性：a -> b, 而 b -> c，那么 a -> c
- 工具类的 happens-before 原则： 
  - 线程安全的容器 get 时一定能看到之前的 put
  - Semaphore
  - CountDownLatch
  - Future
  - CyclicBarrier
  - 线程池



衡量并发安全问题时不能以时间上的运行为依据，只能以 Happens-Before 为原则

