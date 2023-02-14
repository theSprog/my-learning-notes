### DCL 概述

DCL 用于控制有哪些用户可以访问本数据库服务，每一个用户在访问时能够访问哪些数据库，他们对该数据库又有哪些权限等等



#### 管理用户

- 查询用户

  > 所有的用户信息都存放在 mysql 数据库下的 user 表中
  >
  > ```sql
  > use mysql
  > select * from user
  > ```

- 创建用户

  > 只有用户名和主机名一起才能够完整定义一个用户，表示只能在该主机下才能以该用户名访问 MySQL 服务
  >
  > 密码是指为该用户创建的密码
  >
  > 新创建的用户默认没有任何权限（除了 usage 权限）
  >
  > ```sql
  > CREATE USER '用户名'@'主机名' IDENTIFIED BY '密码'
  > -- 例如
  > CREATE USER 'abc'@'localhost' IDENTIFIED BY '123'
  > -- % 代表任意主机均可访问该服务
  > CREATE USER 'abc'@'%' IDENTIFIED BY '123'
  > ```

- ySQL修改用户密码

  > ```sql
  > ALTER USER '用户名'@'主机名' IDENTIFIED WITH mysql_native_password BY '新密码'
  > ```

- 删除用户

  > ```sql
  > DROP USER '用户名'@'主机名'
  > ```



#### 权限控制

MySQL 中定义了许多权限

| 权限名              | 说明               |
| ------------------- | ------------------ |
| ALL, ALL PRIVILEGES | 所有权限           |
| SELECT              | 查询数据           |
| INSERT              | 插入数据           |
| UPDATE              | 修改数据           |
| DELETE              | 删除数据           |
| ALTER               | 修改表             |
| DROP                | 删除数据库/表/视图 |
| CREATE              | 创建数据库/表      |



- 查询用户权限状况

  > % 代表匹配任意主机
  >
  > ```sql
  > SHOW GRANTS FOR'用户名'@'主机名'
  > ```

- 授予用户权限

  > 使用 * （而非 %）匹配任意数据库或者任意表
  >
  > ```sql
  > GRANT 权限列表 ON 数据库名.表名 TO '用户名'@'主机名'
  > -- 例如，多个权限之间用逗号分割
  > GRANT SELECT ON *.* TO '用户名'@'主机名'
  > ```

- 撤销用户权限

  > 使用 * （而非 %）匹配任意数据库或者任意表
  >
  > ```sql
  > REVOKE 权限列表 ON 数据库名.表名 FROM '用户名'@'主机名'
  > ```

  








