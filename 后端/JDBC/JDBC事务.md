### JDBC事务机制

JDBC默认是任意一条DML语句就算一条事务（写入磁盘），但实际开发中我们是将 N 条语句捆绑到一起成为一个事务（即要么他们一起写入磁盘，要么都不写入磁盘），保证这些语句同时成功或者同时失败。



### setAutoCommit

JDBC 的事务默认是自动提交，使用 `setAutoCommit` 设置手动提交，

```java
conn = DriverManager.getConnection("xxx");
conn.setAutoCommit(false);

// 具体 SQL 语句

// 执行到此处，手动提交数据
conn.commit();
// 如果发生异常，回滚
conn.rollback();
```







