### JDBC的本质

JDBC只是 SUN 公司开发的一套数据库接口，真正的驱动文件（例如 `mysql-connector.jar`）需要到指定的官网下载。

使用 JDBC 时需要把 classpath 配置好数据库驱动的所在地





### JDBC 编程六步（增删改查）

#### 1. 注册驱动（告诉 Java 即将要连接的是哪个数据库）

​	以 MySQL 为例，maven 配置

```xml
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>
```



导包，注册驱动。注意 MySQL 中有两个 Driver：

​	`com.mysql.jdbc.Driver` 是 mysql-connector-java **5**中的， 
​	`com.mysql.cj.jdbc.Driver` 是 mysql-connector-java **6**以及以上中的

```java
import com.mysql.cj.jdbc.Driver;

try {
    DriverManager.registerDriver(new Driver());
} catch (SQLException e) {
    e.printStackTrace();
}
```



#### 2. 获取连接（获取 JVM 进程和数据库进程的通道）

MYSQL URL 格式是 `<protocol>://<ip>:<port>/<dbName>?<dbParameter>"`。其中 `dbParameter` 是数据库的各种参数，如时区等等

不同的数据库 URL 不尽相同，但都包含协议，IP和端口之类的字段。

如 Oracle URL 是 `jdbc:oracle:<mode>:@<IP>:<port>:<dbName>`

```java
String url = "jdbc:mysql://localhost:3306/wiki?serverTimezone=Asia/Shanghai";
String user = "root";
String password = "123456";
Connection connection = DriverManager.getConnection(url, user, password);
```



#### 3. 获取数据库操作对象（用于在该数据库执行SQL语句）

获取 Statement 对象，用其执行 SQL 语句

```java
Statement statement = connection.createStatement();
```



#### 4. 执行 SQL 语句（增删改）

JDBC 中的 SQL 语句不需要 `;` 结尾。executeUpdate 方法接受 `insert`、`delete` 和 `update` 相关的 SQL

```java
String sql = "insert into testtable(id, age, comm) values (2, 28, \"abcs\")";
String sql = "delete from testtable where id=1";
String sql = "update testtable set age=18 where id=2";

// 返回值是 int
int count = statement.executeUpdate(sql);
```



#### 5. 处理查询结果（查）

只有 select 语句才会返回数据结果。ResultSet 也是资源，必须也要关闭

```java
String sql = "select * from testtable";
ResultSet resultSet = statement.executeQuery(sql);

if (resultSet != null) {
    try {
        resultSet.close();
    } catch (SQLException e) {
        e.printStackTrace();
    }
}
```



##### ResultSet

ResultSet 是一个特殊的对象，内含一个 `next()` 方法，当下一行合法时返回 `true`，否则返回 `false`，所以取数据时需要先 `next()` 判断是否还有数据

`getString()` 方法获取每一行的列数据，列索引以 1 开始，而不是 0

或者可以以字符串标签的方式获取对应列的数据

```java
while (resultSet.next()){
    String id = resultSet.getString(1);		// 以列索引的方式
    String age = resultSet.getString("age");// 以列标签的方式
    String comm = resultSet.getString(3);
    log.info("id: {}, age: {}, comm: {}", id, age, comm);
}
```

注意的是以标签方式传入的**不是数据库的列名**，而是查询结果集指定的列名

```java
// 将 id 改名 为 id2
String sql = "select id as id2, age, comm from testtable";
resultSet = statement.executeQuery(sql);

String id = resultSet.getString("id2");
```



除了可以以 `string` 类型取出数据，还可以以特定类型取出

```
resultSet.getInt();
resultSet.getFloat();
resultSet.getDouble();
resultSet.getDate();
resultSet.getBlob();
resultSet.getObject();
```



#### 6. 释放资源（JVM 和数据库属于进程间通信）

```java
if(statement != null) {
    try {
    	statement.close();
    } catch (SQLException e) {
    	e.printStackTrace();
    }
}
if(connection != null){
    try {
    	connection.close();
    } catch (SQLException e) {
    	e.printStackTrace();
    }
}
```





### 使用类加载的方式注册 Driver

MySQL 的 Driver 中包含静态代码块，在类加载时就会被初始化

```java
public class Driver extends NonRegisteringDriver implements java.sql.Driver {
    public Driver() throws SQLException {
    }

    static {
        try {
            DriverManager.registerDriver(new Driver());
        } catch (SQLException var1) {
            throw new RuntimeException("Can't register driver!");
        }
    }
}
```

所以不一定需要用 `new` 的方式触发类加载，也可以使用反射

```java
Class.forName("com.mysql.cj.jdbc.Driver");	// 不需要返回值，因为我们只需要类加载时初始化的动作
```



### 使用 PreparedStatement 解决 SQL 注入的问题

数据库中没有 `id='123'` 和 `age='abc'` 的数据

```java
String myid = "123";
String myage = "abc' or '1'='1";

// sql = "select * from testtable where id='123' and age='abc' or '1' = '1'"
// 可以看出 '1' = '1' 恒成立
String sql = "select * from testtable where id='" + myid + "' and age='" + myage + "'";
resultSet = statement.executeQuery(sql);

resultSet.next() // -> true; 表示有数据，但其实是因为 '1' = '1' 成立
```



要想用户信息不参与sql语句的编译，那么必须使用 `java.sql.PreparedStatement`，`PreparedStatement` 是预编译的数据库操作对象，首先对 sql 语句编译，然后再给 sql 传值

使用 `?` 表示占位符，对 sql 语句预编译，返回的 `preparedStatement` 等待接收值。

```java
String preparedSql = "select * from testtable where id=? and age=?";
preparedStatement = connection.prepareStatement(preparedSql);
```

传值时下标从 1 开始，JDBC 中下标都是从 1 开始

```
preparedStatement.setString(1, myid);
preparedStatement.setString(2, myage);

resultSet = preparedStatement.executeQuery();
```

而且由于 `PreparedStatement` 预先编译，属于是一次编译可用于多次执行。相比于 `Statement` 多次编译多次执行，效率更高



### 使用Statement时机

有时候我们需要SQL语句拼接时，仍然需要使用 `Statement` 语句，例如查询支持升序和降序

```sql
order by xxx asc
order by xxx desc
```

上面两个语句末尾的序关系不能用 `?` 占位符实现，只能用SQL语句拼接

