### 起因

之前每一个类都对应一个 Servlet，一旦项目变大，请求路径变多，极易造成 Servlet 类爆炸问题。

而且有一部分类处理的业务有高度的相关性，可以将它们放到同一个类中，提高内聚性

```java
//模板类
@WebServlet({"/dept/list","/dept/save","/dept/edit","/dept/delete","/dept/modify"})
public class DeptServlet extends HttpServlet{
    //模板方法，相当于模仿Controller
    //重写service方法（并没有重写doGet或者doPost)
    @Override
    protected void service(HttpServletRequest request,HttpServletResponse response)
        throws ServletException,IOException{
        // 获取匹配到的路径
        String path = request.getServletPath();
        //封装统一处理
        if("/dept/list".equals(path)){
            doList(request,response);
        }else if("/dept/save".equals(path)){
            doSave(request,response);
        }else if("/dept/edit".equals(path)){
            doEdit(request,response);
        }else if("/dept/delete".equals(path)){
            doDelete(request,response);
        }else if("/dept/modify".equals(path)){
            doModify(request,response);
        }else{
            // 处理错误，然而在这种 WebServlet 中不可能会走到这里
            doErr();
        }
    }
}    
```



WebServlet 支持 `*` 通配符

```java
@WebServlet("/dept/*")

// 这种情况会截取所有 /dept 开头的请求，包括 "/dept" 和 "/dept/abc/def/ghi" 等等
// 但也可能截取到未知请求，如 /dept/error
// 所以需要在封装处理中花一点代码额外处理这部分请求
```

用这种模糊匹配的方式获取的路径信息也稍有不同

```java
// 获取路径的方式要修改为 getPathInfo，表示获取匹配到的 * 的内容
String path = request.getPathInfo();
// 匹配到的路径也要修改为
if("/list".equals(path)){
    doList(request,response);
}
...
```



### 获取路径

此处我们使用的是 `getServletPath`，还有其他的一些路径获取方式

- `getServletPath`：获取能够与 `urlPattern` 中匹配的路径，注意是**完全匹配**的部分，`*`的部分不包括
- `getPathInfo()`：与`getServletPath()`获取的路径互补，能够得到的是 `urlPattern` 中 `*`  的路径部分，不存在则返回 `null`
- `getContextPath()`：获取项目根路径
- `getRequestURI()`：获取项目根路径到地址结尾的内容（包括项目根路径）
- `getRequestURL()`：获取请求的地址链接（浏览器中输入的地址）
- `getServletContext().getRealPath("/")`：获取 `/` 在机器中的实际地址
- `getScheme()`：获取的是使用的协议 (http 或 https)
- `getProtocol()`：获取的是协议的名称 (HTTP/1.11)



假设项目根路径是 `/abc`，`WebServlet("/hello/*")`，请求路径是 `/abc/hello/hello2/hello3`

- `getServletPath`：/hello
- `getPathInfo()`：/hello2/hello3
- `getContextPath()`：/abc
- `getRequestURI()`：/abc/hello/hello2/hello3

