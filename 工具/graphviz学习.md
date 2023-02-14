### 基础图形绘制

dot接受dot语言格式的文件，dot语言主要有三部分：graph，node，edge, 属性使用 **attr=value** 的形式使用

node与edge是关键字，可以改变默认的 node 与 edge 的属性

// 表示单行注释，/* */ 表示块注释

```shell
// digraph 表示有向图
// graph 表示无向图
digraph base_flow {
    // 此处 label 表示图名称
    label = "graphviz使用流程";
    
    // 将 node 的默认形状改为 box
    node[shape=box];
    
    // 定义 edge
    graph_attr -> node_edge_attr -> node_edge_added -> custom_attr;
	
    // 为某个节点更改属性，使用 [attr=value]
    graph_attr[label="1. 定义digraph的属性"];
    node_edge_attr[label="2. 定义node、edge的属性"];
    node_edge_added[label="3. 添加node、edge"];
    custom_attr[label="4. 定义特定node，edge的属性"];
}

// 编译
$ dot -T<imgType> <fileName> -o <imgName>
// 例如
$ dot -Tpng a.dot -o a.png
```

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061710474.svg)



### 绘制属性

#### 节点形状

节点形状默认为椭圆（ellipse），并且用节点名称作为 label 值

- label，表示节点所显示的字面值
- shape，节点所显示的形状

<img src="https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061739606.png"  />

- peripheries，表示节点外套多边形层数

  ```shell
  digraph base_flow {
      a[label="1", peripheries=3];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061742908.svg)

- orientation，表示节点顺时针旋转度数，以度为单位

  ```shell
  digraph base_flow {
      a[label="1", shape=box, orientation=120];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061747229.svg)

- sides，多边形形数，需要和 `shape=polygon` 配合使用

  ```shell
  digraph base_flow {
      a[label="1", shape=polygon, sides=10];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061749689.svg)

- skew，偏度，表示多边形顶部向右偏离的程度，以浮点数计算，通常范围为 **[-1.0, 1.0]**

  ```shell
  digraph base_flow {
      a[label="1", shape=polygon, sides=3, skew=0.5];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061753060.svg)

- distortion，扭曲度，表示图形底部和顶部的比例，0表示上下完全相等，越大顶部越宽

  ```shell
  digraph base_flow {
      a[label="1", shape=polygon, sides=6, distortion=1];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061757460.svg)

- shape=record或Mrecord，相等的概念，表示用记录组成的节点，唯一的区别就是后者有圆角。

  - record的 label 标签表示一个节点内记录的排布方式

  ```shell
  digraph structs {
   struct [shape=record,label="hello\nworld | {b | {c| d |e} | f} | g | h"];
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061806943.svg)

  - `graph[rankdir=TB]` 整体图片垂直布局，`graph[rankdir=LR]`，整体图片水平布局（LR=left to right, TB=top to bottom）
  - 直接使用 rankdir=TB 表示所有元素都用 TB 布局，rankdir=LR 表示所有元素都用 LR 布局
  - 若给某个记录以一个 portname，那么其他节点可以引用此记录

  ```shell
  // 模仿二叉树
  
  digraph structs {
      node[shape=record];
      graph[rankdir=LR];
  	
  	// left 命名为 f0，data 命名为 f1，right 命名为 f2
      struct1[label="<f0> left|<f1> data|<f2> right"];
      struct2[label="<f0> left|<f1> data|<f2> right"];
      struct3[label="<f0> left|<f1> data|<f2> right"];
      
      // 使用 : 来表示选择
      struct1:f0 -> struct2:f1;
      struct1:f2 -> struct3:f1;
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061813989.svg)

- 自定义节点图片

  ```shell
  graph pic_test {
      pic[shape=none, label="xyz", imagepath="<img_folder>", image="<img_name>"];
  }
  ```

  

-  待续



#### Label属性

默认的 label 就是节点的名字，但我们可以更改 label 来更改节点显示的名称。当有多个节点都叫同一个名称的时候，就必须显示指明节点的 label 了，因为节点的名称必须唯一，而 label 可以不唯一。



节点可以使用 `label=<...>` ，其中 `...` 是 html 语法



#### Style属性

style 属性用于修改节点的外观，当前，支持8种类型的 style：`filled, invisible, diagonals, rounded. dashed, dotted, solid, bold`

- filled。 此值指示应填充节点的内部。使用的颜色是 fillcolor 定义的，若 fillcolor 属性未定义，则使用 color 属性的颜色

  ```shell
  digraph G {
    // 节点默认是蓝色
    node [color=blue]
    node1 [style=filled] 
    node2 [style=filled, fillcolor=red] 
    node0 -> node1 -> node2
  }
  ```

  ![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061835868.svg)



#### port 属性

节点的 port 属性是指节点连接另一个节点的线条端点位置，端口的位置有8种，分别为节点的东、南、西、北、东南、东北、西南、西北，属性的值分别为`e, s, w, n, se, ne, sw, nw`

有两种类型的 port 属性:

- 一种使指定**源节点**的端点位置，使用 `tailport` 属性，如下脚本指定 a节点的端点位置为东：

```cpp
digraph G {
    a -> b [tailport = e];	// 从源节点 east 出发
}
```



![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061854690.svg)



- 一种指定**目的节点**的端点位置，使用`:pos`语法，如下脚本指定b节点的端点位置为西：

```cpp
digraph G {
    a -> b:w;	// 指向目标节点的 west
}
```

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061855335.svg)



也可以通过上述语法指定 record 形状的域字段（如f1）的端点位置：

```cpp
digraph G {
    a -> b:f1:w;	// 指向 f1 的 west
}
```

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201061854306.svg)





### 子图绘制

子图使用 `subgraph` 关键字后跟子图的名称，需要注意的是子图必须用 `cluster` 作为开头



#### 连接子图

当要链接子图时需要使用 `compound=true` ，使用一个子图A的节点连接到子图B的节点，若需要从子图A出发（而不是从节点出发），则需要设置edge属性：`ltail=A`，同理若需要到达子图B（而不是到达节点），则需要设置edge属性：`ltail=B`

```cpp
digraph G {
  compound=true;
  subgraph cluster0 {
    a -> b;
    a -> c;
    b -> d;
    c -> d;
  }
  subgraph cluster1 {
    e -> g;
    e -> f;
  }
  b -> f [lhead=cluster1];
  d -> e;
  c -> g [ltail=cluster0,lhead=cluster1];
  c -> e [ltail=cluster0];
  d -> h;
}
```

![](https://bucket01-1259777572.cos.ap-chengdu.myqcloud.com/img/202201082008701.svg)