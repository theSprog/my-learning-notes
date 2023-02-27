## Java7 的经典 HashMap

java7 使用的 HashMap 属于经典的 数组+链表 形式，哈希碰撞时采用链表解决。



### 为什么初始容量（桶的个数）必须是 2 的幂次

刚创建 hashmap 时并没有开辟空间，只有第一次往里面 `put` 时才会申请内存。当往里面 `put` 东西的时候，计算完 hash 值后必须选定应该存放在 `[0, n-1]` 的哪一个桶中。

> 常规做法是 hash % n 得出桶索引号，然而这样做有两个缺点：
>
> - 在 Java 中负数取模为负数，而负数不能够作为索引号，因此必须再检测一次结果是否为负数，效率低
> - 取模运算本质上是做除法，而除法较慢

假设桶的个数是 `length`，如果它是 2 的幂次的话，那么直接 `hash & (length-1)` 即可直接得出桶的索引号，而且减法和与运算都比除法快，这一切的前提都在于 `length` 必须是 2 的幂次。

还有另外一个原因，那就是在扩容迁移的时候不需要再重新计算哈希以定位新的位置了。扩容后，元素新的位置要么在原脚标位，要么在 原脚标位+扩容长度 这么一个位置



### 一切问题的根源：transfer

transfer 的作用是将所有的元素从当前的桶迁移到新的桶

```java
void transfer(Entry[]newTable, boolean rehash){
    int newCapacity = newTable.length;
    for (Entry<K,V> e : table){	// 对每一个桶的元素
        while(null != e){
            Entry<K,V> next = e.next;
            if (rehash){
            	e.hash = null == e.key ? 0 : hash(e.key);
            }
            int i = indexFor(e.hash,newCapacity);
            e.next = newTable[i]
            newTable[i] = e;
            e = next;
        }
    }
}
```

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220190121176.png" alt="image-20230220190121176" style="zoom: 25%;" />

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220190148014.png" alt="image-20230220190148014" style="zoom: 25%;" />

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220190214729.png" alt="image-20230220190214729" style="zoom: 25%;" />

可以看出，由于使用头插法，扩容后的元素顺序和原来的顺序是相反的。



### 死锁问题

HashMap 的实现本来就不是线程安全的，在多线程环境下可能会出现死锁问题。这里的关键就是头尾调换问题。

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220190918627.png" alt="image-20230220190918627" style="zoom: 67%;" />

- 假设有两个线程，它们都在进行扩容操作，线程1的指向如图，线程2已经扩容完毕

- 然后线程 1 被调度

  > ```java
  > // 相当于进行一次 while 循环
  > newTable[i] = e;
  > e = next;
  > next = e.next;
  > ```
  >
  > 执行之后程序指向关系如图
  >
  > <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220191305117.png" alt="image-20230220191305117" style="zoom: 67%;" />

- 然后线程 1 继续运行，将当前 e 指向元素摘下，放入自己申请的内存里

  > <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220191740744.png" alt="image-20230220191740744" style="zoom: 67%;" />

- 然后 `e.next = newTable[i]`，从而 `key(3)` 的下一个元素指向 `key(7)`，而 `key(7)` 的 `next` 本就指向 `key(3)` 的，此时出现环形链表

  > <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20230220192050487.png" alt="image-20230220192050487" style="zoom: 80%;" />

- 如果此时在该桶中查找 `key(11)` 的话，那么会出现无穷循环（infinite loop）



### 安全隐患

我们完全可以精心构造多个对象，使得它们的 hash 值相同，这样就使得 HashMap 退化为链表。

在 Tomcat 中它使用一个 HashMap 存储 http 请求，而黑客可能构造成千上万的请求，从而让 Tomcat 发生频繁的哈希碰撞，构成 Dos 攻击，将服务器攻击瘫痪。





## 面试题

### 如何解决 hash 冲突

- 在 Java7 时 hashmap 采用的是 数组+链表 的形式，当两个 key 对应的数组索引是同一处时，会采用尾插法将新加入的元素插入到链表的尾部
- 在 Java8 时为了避免链表过长导致查询效率下降，当链表长度 `>8` 并且数组长度 `>=64` 时会将当前链表转换为红黑树



### HashMap 和 HashTable 的区别

- HashTable 是线程安全版本的 HashMap 
- HashTable 涉及到线程不安全的方法都统一加上 `synchronized` 关键字保证同步
- HashMap 可以允许 null 作为 key，而 HashTable 不允许
- HashTable 直接选择 key 的 hashcode 取模作为数组索引，而 HashMap 对 hashcode 进行再次散列，从而避免 key 的分布不均匀的问题



### HashMap 什么时候扩容

当 HashMap 中的元素超过临界值的时候会触发扩容，临界值 `threshold = loadfactor * capacity`，负载因子默认是 `0.75`。每次扩容后新容量是原来的两倍。

由于这种动态扩容的存在，因此使用时如果预先对数据量有个估计的话，应该手动指定初始容量，避免频繁扩容造成性能损失。



### 扩容因子为什么是 0.75

扩容因子过大，则会导致即使碰撞很频繁也不会扩容；而扩容因子过小会导致频繁扩容，效率降低。因此扩容因子的选择本质上就是碰撞概率和空间利用率之间的一个权衡。

至于为什么选择 0.75，是因为经过测试这个数值能够在两者之前取得一个恰到好处的平衡，这个数值好像是和泊松分布有关。











