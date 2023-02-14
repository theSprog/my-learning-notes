# 移植FFI

在说明如何对FFI进行移植之前需要先说明FFI的实现原理。JEP424是外部函数访问+本地内存，但是实际上需要移植的内容只有外部函数访问，对于本地内存的操作并不需要修改。

从java中调用native方法叫做downcall，而从native方法中调用java方法叫做upcall，下面通过downcall和upcall的两个简单例子，来看看实现原理和移植FFI需要修改哪些地方。



## downcall

下面是一个典型的使用例子：

![image-20220821185848728](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220821185848728.png)

首先需要在加载器已加载的动态库中进行符号搜索，上面搜索了C函数`sayHello`，返回的类型是一个`MemorySegment`对象，表示了内存中的一段内存，不过也可以当作一个地址来看待，所以相当于是返回了目标符号的地址。

Linker.nativeLinker会返回对应平台实现的Linker，然后通过`downcallHandle`创建了一个`MethodHandle`，里面传入了两个参数，一个是刚刚返回的符号地址*target*，另外一个则是使用` FunctionDescriptor.ofVoid(JAVA_INT, JAVA_FLOAT)`构造出来的函数表述符，这是个工厂方法，很明显传入的参数和`void sayHello(int a, float b)`是对应的，传入int、float返回void。JAVA_INT、JAVA_FLOAT是MemoryLayout类型的常量，MemoryLayout用于描述内存布局或者描述类型，包含了类型的长度和对齐要求，MemoryLayout有4个子类，比较常用的就是`ValueLayout`和`GroupLayout`，分别用于表示基本类型和聚合类型。

返回回来的`MethodHandle`非常普通，调用方式和其他的`MethodHandle`没有不同，其实此时大概已经可以猜测出来FFI的工作原理了，就是将参数传递给返回的*mh*对应的方法（underlying method），然后由这个方法进行一些代理的工作，比如正确地处理参数的输入和返回值的输出，这些需要按照对应平台的C abi来进行处理，最终这个方法会调用传入的*target*所指向的C函数。

那么进入`downcallHandle`里面看看发生了：

![image-20220820195032115](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820195032115.png)

首先使用`downcallHandle`创建了一个`MethodHandle`，然后使用`bindTo`将符号地址作为第一个参数进行绑定，进一步往里面：

![image-20220820195001643](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820195001643.png)

这里以描述符为主键从缓存中获取MethodHandle，这样可以防止MethodHandle多次创建，在待会就可以看到这个MethodHandle所引用的方法其实是动态生成的，在第一次创建的时候事实上都会进入到`arrangeDowncall`中。

这是移植任务中主要需要注意的地方，其实现是与架构和系统相关的，所以在`src/java.base/share/classes/jdk/internal/foreign/abi`目录下，继续进入查看：

![image-20220820155435004](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820155435004.png)

上面所展示的是在RV64下的尝试性的实现，在其他平台下，`arrangeDowncall`的实现并没有太多不同。首先会创建一个Binding对象，这个对象十分重要，会根据这个对象来进行创建上被调用的MethodHandle，所以来看看这个对象内部的字段：

![image-20220820160003766](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820160003766.png)

后三个字段是为了移植到RV64暂时添加的，并不重要。第二个字段用以标明是否需要返回在内存中，在C abi下如果一个结构体超过了16B那么就需要在内存中进行返回，对应的，第一个参数就会替换为返回地址。

再来看第一个字段，第一个字段的对象内部有许多字段：

![image-20220820161309277](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820161309277.png)

最为重要的就是returnBindings和argumentBindings这两个，这两个字段都是List类型的，在后续的操作中主要就是根据这两个列表来动态生成MethodHandle引用的方法。从上图可以看到MethodHandle一共有3个参数，为什么是3个？还记得上面提到过的那个bindTo么？它就是给那个符号地址预留的。

为了防止嵌入的层次过深，所以暂时不进入到getBindings进行查看，先来看看argumentBindings的每个元素好了。可以看到每个元素对应的还是一个列表，这个列表代表着一系列的操作，这些操作是对一个操作数栈来进行的，可以假设传入的参数是操作数栈的栈底，对于符号地址（MemorySegment）来说其实现了Addressable接口，然后Biding$UnboxAddress会对应生成如下操作：

![image-20220820163220671](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820163220671.png)

也就是将传入的MemorySegment转为一个64位地址，将一个数据类型的java表示转为native表示的过程被称作为unbox，反之则是box。

第二个操作是Binding$VMStore，在进行downcall的时候会对应生成：

![image-20220820164837670](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820164837670.png)

里面的内容看起来好像这个操作会负责将栈中的操作数放入到寄存器`t4`，其实并不是，它生成的操作只会将操作数放入到slot中，真正进行这个操作的其实是downcallStub，这是一段动态生成的汇编代码，具体内容在下面进行介绍，它负责将java传入的参数进行排列使之能够符合C abi然后调用C函数，并将C函数的返回值进行处理返回到java。

继续回到上面：

![image-20220820155435004](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820155435004.png)

真正使用了bindings的地方是在`getBoundMethodHandle`，不过由于其相对比较长所以不便于展示完整的截图，所以看几个关键的地方就可以了：

![image-20220820165830411](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820165830411.png)

首先通过工厂方法make创建了一个NativeEntryPoint对象，在创建它的过程中同时也会创建downcallStub，上面提到过downcallStub真正负责将参数移入到正确的位置。创建downcallStub的调用为NativeEntryPoint.makeDowncallStub，其对应了`src/hotspot/share/prims/nativeEntryPoint.cpp`中的JNI调用，这个函数会调用`src/hotspot/cpu`的`make_downcall_stub`。

toStorageArray(argMoves)为：

![image-20220820183032220](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820183032220.png)

而return部分则是空的。

随后通过nep创建了一个NativeMethodHandle，不过这个MethodHandle并不会返回到用户手上，而是再进行一个包装：

![image-20220820184326134](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820184326134.png)

`USE_SPEC`默认为true，BindingSpecializer.specialize就是在之前提到过动态生成方法的地方，而下面做的事情也差不多，不过是直接对进行了解释，每个Binding类有一个抽象方法interpret。两者干的事情应该是差不多的，不过不清楚到底有什么区别，移植的话对它们的原理有了解即可，不需要对这部分的代码进行修改。

再次返回到`arrangeDowncall`：

![image-20220820155435004](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820155435004.png)

这次来查看getBindings的内部实现：

![image-20220820194207085](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820194207085.png)

getBindings内部和其他平台是类似的，没有什么区别，有区别的地方是`BoxBindingCalculator`和`UnboxBindingCalculator`这两个类，这两个类都实现了`getBindings`方法，且保存当前的storage分配状态，`getBindings`方法用于根据MemoryLayout来分配storage，分配的storage包括寄存器、stack slot、其他内存位置。之前在创建FunctionDescriptor的时候也提到过MemoryLayout，这里使用的MemoryLayout就是构造的时候传入的。

java代码部分主要的移植工作就是在BindingCalculator类里面，其他的部分都近乎可以复制其他平台的代码。

继续往里面看：

![image-20220820195457995](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820195457995.png)

首先使用了`TypeClass.classifyLayout(layout)`根据MemoryLayout对处理方式进行分类，在我的实现中大致将参数分为了下面的类型：

![image-20220820195738232](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820195738232.png)

下面是UnboxBindingCalculator的实际处理：

![image-20220820200408681](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820200408681.png)

再次返回到`arrangeDowncall`：

![image-20220820155435004](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820155435004.png)

此时，MethodHandle已经构造完毕，然后就会返回到用户，用户调用了invoke方法之后就会开始执行之前动态生成的方法，动态生成的方法对用户输入参数完成转换之后就会调用NativeEntryPoint.make生成的downcallStub，在downcallStub中会真正调用链接库中的指定符号。

在BindingSpecializer.specialize中动态生成的逻辑和移植没有什么关系，是通用的，所以修改生成downcallStub的代码`src/hotspot/cpu/riscv/downcallLinker_riscv.cpp`就好了，这里面基本上也可以参考x86或者aarch64中的实现，需要修改的主要就是需要实现`DowncallStubGenerator::generate()`，不过其实里面涉及了很多关于栈空间开辟、虚拟机状态转换、安全点检查之类的代码都是可以参考x86和aarch64实现的。

动态生成的方法会将参数传递给downcallStub，但是由于downcallStub使用的是java abi，参数传递方式和C abi有不同，所以在downcallStub中需要将其接受到的参数重新排列使其可以满足C abi。其实动态生成的方法只负责将参数处理之后放在指定位置然后调用downcallStub，downcallStub中将自己的参数和传入的storage进行配对，然后通过`ArgumentShuffle::generate`生成移动代码。



## upcall

除了使用downcall，从java代码中调用外部的C函数之外，还可以让外部的函数调用java中的函数，下面是一个典型的例子：

![image-20220820212524292](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820212524292.png)

这里创建了一个upcallStub然后作为参数，也就是函数指针传递给了调用native的MethodHandle。由于已经解释过了downcall，而upcall和downcall其实非常相似，所以可以跳过很多的内容，看看如何创建upcallStub就好了，upcallStub负责接收C传入的参数，并使之能够按照java abi来放置，然后调用java方法，随后处理java方法的返回值，将其按照C abi来放置进行返回。

在调用了`LINKER.upcallStub`之后会进入到平台对应Linker的arrangeUpcall中：

![image-20220820214503280](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820214503280.png)

处理Bindings的方式和downcall是类似的，都是按照输入参数和返回值对storage进行分配，在进行移植的时候只需要按照C abi仿照x86或者aarch64的实现来实现就行了。在上面的实现中，我进行了一下修改所以和现有的代码有些不同。

进入到`UpcallLinker.make`中会发现，这里和`DowncallLinker.getBoundMethodHandle`的逻辑是反过来的，这里是先自动生成了方法，然后将自动生成的方法的MethodHandle作为参数，传递给了`makeUpcallStub`，是一个native方法：

![image-20220821195702933](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220821195702933.png)

![image-20220821195739829](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220821195739829.png)

进入到JNI中，`src/hotspot/share/prims/upcallLinker.cpp:UL_MakeUpcallStub`，可以看到传入进去的MethodHandle在经过一些处理之后成为了入口`entry`：

![image-20220820220426976](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820220426976.png)

`UpcallLinker::make_upcall_stub`会接受entry，并且在参数传递处理完毕之后调用它：

![image-20220820220718124](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220820220718124.png)

到这里位置和downcall的情况都差不多，不过由于upcall中java方法可能抛出异常，所以还需要有一个异常处理器，处理被抛出的异常。因为会抛出异常所以还需要进行栈遍历，所以其中还有关于栈遍历的一些处理，`src/hotspot/cpu/riscv/frame_riscv.hpp`和`src/hotspot/cpu/riscv/frame_riscv.cpp`也需要跟着进行一些修改。



## 总结修改位置

上面对FFI的实现原理进行了一个概览，这里对实现FFI需要修改的位置进行一下总结：

java:

1. `src/java.base/share/classes/jdk/internal/foreign/abi`目录下创建一个与架构对应的文件夹，在架构文件夹下创建一个与系统对应的文件夹，里面的内容可以参考x86或者aarch64实现来进行。
2. `src/java.base/share/classes/jdk/internal/foreign/CABI.java`添加对应的CABI对象。
3. `src/java.base/share/classes/jdk/internal/foreign/abi/SharedUtils.java`凡是有出现过其他平台的地方，添加上需要实现的平台，比如：

```java
public static Linker getSystemLinker() {
  return switch (CABI.current()) {
    case Win64 -> Windowsx64Linker.getInstance();
    case SysV -> SysVx64Linker.getInstance();
    case LinuxAArch64 -> LinuxAArch64Linker.getInstance();
    case MacOsAArch64 -> MacOsAArch64Linker.getInstance();
    case LinuxRV64 -> LinuxRV64Linker.getInstance();
  };
}
```



hotspot:

1. src/hotspot/cpu/riscv/frame_riscv.hpp
2. src/hotspot/cpu/riscv/frame_riscv.cpp
3. src/hotspot/cpu/riscv/foreignGlobals_riscv.hpp
4. src/hotspot/cpu/riscv/foreignGlobals_riscv.cpp
5. src/hotspot/cpu/riscv/downcallLinker_riscv.cpp
6. src/hotspot/cpu/riscv/upcallLinker_riscv.cpp