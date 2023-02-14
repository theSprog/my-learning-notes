### 通信过程

一次网络通信的基本过程如下，右边是服务器端，左边是客户端

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220927152903927.png" alt="image-20220927152903927" style="zoom:80%;" />



#### Server

`ServerSocket` 专门指为服务器建立 socket，默认指本地环回 IP

我们可以使用 `socket` 提供的 `InputStream/OutputStream`，但是这种流是字节流，只能以字节的 形式操作。

而 `Reader/Writer` 提供字符流，可以以 `String` 的形式操作。

还可以再在外部包装一层 `Buffer`，从而形成缓冲区，减少系统调用频率，提高效率。但是要想控制发送数据就必须手动调用 `flush`，否则的话默认流满了再发送

另外我们此处暂时使用单线程，意味着只能等这一个连接退出后才能处理下一个连接

```java
public class Server {
    public static void main(String[] args) {
        // 自动执行 bind, 绑定到本机的 8888 端口
        try(ServerSocket serverSocket = new ServerSocket(8888)) {
            Socket clientSocket = serverSocket.accept();
            var writer = new BufferedWriter(
                    new OutputStreamWriter(clientSocket.getOutputStream()));
            var reader = new BufferedReader(
                    new InputStreamReader(clientSocket.getInputStream()));
            
            while (true){
                // 服务器首先发送
                writer.write("what's your name\n");
                writer.flush();
                
                // 等候客户端响应, 注意末尾的 \n 不会被接受
                // 注意如果客户端已关闭，向已关闭的流读数据会抛出异常
                String rep = reader.readLine();
                System.out.println(rep);
                
                // 回复客户端，手动在末尾加上 \n，以供客户端识别
                writer.write("hello " + rep + "\n");
                writer.flush();
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}

```



#### Client

`Socket` 需要两个参数：服务器 IP 地址和端口号，一旦创建就会自动执行 `connect`

```java
public class Client {
    public static void main(String[] args) {
        try (Socket server = new Socket("127.0.0.1", 8888)) {
            var writer = new BufferedWriter(new OutputStreamWriter(server.getOutputStream()));
            var reader = new BufferedReader(new InputStreamReader(server.getInputStream()));
            var consoleReader = new BufferedReader(new InputStreamReader(System.in));
            while (true){
                // 由于服务器先发送数据，所以此处监听，将末尾的 \n 丢弃
                String rep = reader.readLine();
                System.out.println(rep);
                
                // 获取用户控制台输入，并且切记要在末尾加上 \n，否则 readLine 不会返回
                String msg = consoleReader.readLine();
                writer.write(msg + "\n");
                writer.flush();
                
                // 获取服务端响应
                rep = reader.readLine();
                System.out.println(rep);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

注意由于我们使用 `readLine` 获取字符流，而 `readLine` 只有遇到 `\n` 或者 `EOF` 才会返回，故每次发送信息都必须手动在信息加上 `\n`，才能使得另一端的 `readLine` 获取到信息，而不是永远阻塞





### BIO 模型

假设我们要构建一个多人聊天室，用户可以输入内容，然后广播给所有在线用户



#### Server 

##### Server 请求接收器

服务器使用 Acceptor 专门监听请求，一旦发生请求连接，就派生一个 线程去处理该请求

```java
public void start() {
    try {
        // 绑定监听端口
        serverSocket = new ServerSocket(DEFAULT_PORT);
        System.out.println("启动服务器，监听端口：" + DEFAULT_PORT + "...");

        while (true) {
            // 等待客户端连接
            Socket socket = serverSocket.accept();
            
            // ****
            // 创建ChatHandler线程，该线程用到的参数由构造函数创建
            // ****
            new Thread(new ChatHandler(this, socket)).start();
        }

    } catch (IOException e) {
        e.printStackTrace();
    } finally {
        close();
    }
}

public void close() {
    if (serverSocket != null) {
        try {
            serverSocket.close();
            System.out.println("关闭serverSocket");
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
```



##### Handler 请求处理

由于我们使用多线程方式处理请求，因此对应的类必须实现 `Runnable` 接口，即重写 `run` 方法，里面是请求处理逻辑

```java
@Override
public void run() {
    try {
        // 存储新上线用户
        server.addClient(socket);

        // 读取用户发送的消息
        BufferedReader reader = new BufferedReader(
            new InputStreamReader(socket.getInputStream())
        );

        String msg = null;
        while ((msg = reader.readLine()) != null) {
            String fwdMsg = "客户端[" + socket.getPort() + "]: " + msg + "\n";
            System.out.print(fwdMsg);

            // 检查用户是否准备退出，一旦用户发送特定的消息 "QUIT" 就代表退出
            if (server.checkReadyToQuit(msg)) {
                break;
            }
            
            // 将消息转发给聊天室里在线的其他用户
            server.forwardMessage(socket, fwdMsg);
        }
    } catch (IOException e) {
        e.printStackTrace();
    } finally {
        try {
            server.removeClient(socket);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
```



#### Client

##### Client 服务器

我们的 client 应该完成两件任务：

- 监听用户输入
- 监听服务器是否有数据到来

并且这两件事情不能同步，比如不能先监听用户输入，用户输入完成后再监听服务器数据，这两件事情应该是异步的，所以也要使用多线程



##### Handler 处理用户输入

```java
// UserInputHandler.java

// 该类实现了 Runnable 接口
@Override
public void run() {
    try {
        // 等待用户输入消息
        BufferedReader consoleReader =
            new BufferedReader(new InputStreamReader(System.in));
        while (true) {
            String input = consoleReader.readLine();

            // 向服务器发送消息
            chatClient.send(input);

            // 检查用户是否准备退出
            if (chatClient.readyToQuit(input)) {
                break;
            }
        }
    } catch (IOException e) {
        e.printStackTrace();
    }
}
```



##### Client 监听服务器数据

```java
// client.java

public void start() {
    try {
        // 创建socket
        socket = new Socket(DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT);

        // 创建IO流
        reader = new BufferedReader(
            new InputStreamReader(socket.getInputStream())
        );
        writer = new BufferedWriter(
            new OutputStreamWriter(socket.getOutputStream())
        );

        // 处理用户的输入，传入客户端以用于向服务器发送数据
        new Thread(new UserInputHandler(this)).start();

        // 读取服务器转发的消息
        String msg = null;
        while ((msg = receive()) != null) {
            System.out.println(msg);
        }
    } catch (IOException e) {
        e.printStackTrace();
    } finally {
        close();
    }
}
```



#### 使用线程池

![image-20220927200709659](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220927200709659.png)

在构造函数中创建线程池

```java
// 成员变量
private ExecutorService executorService;

// 构造函数
executorService = Executors.newFixedThreadPool(3);
```



将直接使用线程转为使用线程池

```java
new Thread(new ChatHandler(this, socket)).start();
// 改为下面这句
executorService.submit(new ChatHandler(this, socket));
```

使用线程池后，当线程池无空闲线程时，新线程会一直阻塞，直到另一个线程退出，空出 worker



### NIO

NIO 可以

- 使用 Channel 代替 Stream
- 可以在一个线程里面处理多个 Channel I/O
- 使用 Selector 监控多条 Channel 
- 注意的是，Channel 既可以读数据，也可以写数据



#### 使用 NIO 拷贝文件

使用 NIO 处理文件时对应的类是 `FileChannel`。

传统的 Stream 操作的对象是 `byte[]`，但是`Channel ` 操作的对象是 `Buffer`，有两种 `Buffer`：`ByteBuffer` 和 `CharBuffer`，分别对应字节流和字符流

`Buffer` 它有两种模式，读模式和写模式，默认是读模式，使用 `flip()` 之后可以翻转为写模式

```java
public void copyFile(File source, File target) {
    FileChannel fin = null;
    FileChannel fout = null;

    try {
        // 我们希望从 fin 读数据，然后写入到 fout 中
        fin = new FileInputStream(source).getChannel();
        fout = new FileOutputStream(target).getChannel();
		
        // 类似于传统的分配数组，传入长度
        ByteBuffer buffer = ByteBuffer.allocate(1024);
        // 将数据读入 buffer 中，-1 代表已无数据
        while (fin.read(buffer) != -1) {
            // 模式反转，改为写模式
            buffer.flip();
            
            // 由于每一次写不一定写完，buffer 中可能残留未写完的数据，使用 hasRemaining 判断
            while (buffer.hasRemaining()) {
                // 如果未写完，就一直尝试写
                fout.write(buffer);
            }
            // 使用 clear 将数据清空，腾出空闲空间
            buffer.clear();
        }
    } catch (Exception e) {
        e.printStackTrace();
    } finally {
        // buffer 不用关闭, Channel 必须关闭
        close(fin);
        close(fout);
    }
}
```



#### transferTo

transferTo 可以将一个通道内的数据转移至另一个 **可写入** 的通道，但不一定一次就能够完成该操作，取决于通道的性质和状态。

- 参数值
  - long position：源文件的游标，即从此处开始传输，如果给定的位置大于源文件的大小，则不传输
  - long count：需要传输的字节数
  - target：可写入的另一个通道

- 返回值是传输的字节个数，有可能是 0

```java
long transferred = 0L;
long size = fin.size();	// 当前文件的字节数大小
while (transferred != size) {
    transferred += fin.transferTo(0, size, fout);
}
```



#### Selector

Selector 一般称为选择器 ，当然你也可以翻译为多路复用器 。它是 Java NIO 核心组件中的一个，用于检查一个或多个NIO Channel（通道）的状态**是否处于可读、可写** 状态，如此可以实现单线程管理多个 channels，也就是可以管理多个socket

<img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20221001185124238.png" alt="image-20221001185124238" style="zoom:67%;" />