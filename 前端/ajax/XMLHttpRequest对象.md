### XMLHttpRequest

#### 创建对象

```javascript
var xhr = new XMLHttpRequest();
```



#### XMLHttpRequest对象

- #### 方法

| 方法                               | 描述                                                         |
| ---------------------------------- | ------------------------------------------------------------ |
| abort()                            | 取消当前请求                                                 |
| getAllResponseHeaders()            | 返回头部信息                                                 |
| getResponseHeader()                | 返回特定的头部信息                                           |
| open(method,url, async, user, pwd) | method: 请求类型（GET或POST）<br />url: 文件位置<br />async: true(异步)或false(同步)<br />user: **可选的**用户名称<br />psw: **可选的**密码 |
| send()                             | 将请求发送到服务器，用于**GET**请求                          |
| send(string)                       | 将请求发送到服务器，用于**POST**请求                         |
| setRequestHeader()                 | 向要发送的报头添加标签/值对                                  |



- 属性

| 属性               | 描述                                                         |
| ------------------ | ------------------------------------------------------------ |
| onreadystatechange | 定义当readyState属性发生变化时被调用的函数                   |
| readyState         | 保存XMLHttpRequest的状态。<br />0：请求未初始化<br />1：服务器连接已建立<br />2：请求已收到<br />3：正在处理请求<br />4：请求已完成且响应已就绪 |
| responseText       | 以字符串返回响应数据                                         |
| responseXML        | 以XML数据返回响应数据                                        |
| status             | 返回请求的状态号（如 200，404）                              |
| statusText         | 返回状态文本（如"OK"或"Not Found")                           |



#### XMLHttpRequest使用

```javascript
// 1. 创建对象
var xhr = new XMLHttpRequest();

// 2. 注册回调
xhr.onreadystatechange = function(){
    // 此处暂且打印状态变化
    console.log(xhr.readState)
    
    if(xhr.readState == 4) {
        // 打印状态码
        console.log(xhr.status)
        
        if(xhr.status == 200){
            // 获取id对应盒子，并更新内部 HTML
            document.getElementById("xxx").innerHTML = xhr.responseText
        }
    }
}

// 3. 开启通道, open只是浏览器和服务端建立连接，还没有发送请求
// user 和 pwd 有些服务器资源需要身份认证，选填。
// async 设置为 true 表示异步，false 表示同步
xhr.open("GET", "url", true)

// 4. 发送请求
xhr.send()
```

