### maven 依赖

加入 MyBatis 和 generator 依赖

```xml
<!-- 集成mybatis -->
<dependency>
    <groupId>org.mybatis.spring.boot</groupId>
    <artifactId>mybatis-spring-boot-starter</artifactId>
    <version>2.1.3</version>
</dependency>

<!-- 集成pageHelper分页助手 -->
<dependency>
    <groupId>com.github.pagehelper</groupId>
    <artifactId>pagehelper-spring-boot-starter</artifactId>
    <version>1.4.3</version>
</dependency>



<!-- 在plugins中集成 generator -->
<plugin>
    <groupId>org.mybatis.generator</groupId>
    <artifactId>mybatis-generator-maven-plugin</artifactId>
    <version>1.4.0</version>

    <configuration>
        <!-- 此处写 generator-config.xml 配置文件路径 -->
        <configurationFile>src/main/resources/generator/generator-config.xml</configurationFile>
        <overwrite>true</overwrite>
        <verbose>true</verbose>
    </configuration>

    <!--不要删除这个依赖，否则会发生 "提示找不到DB的驱动jar" 的错误-->
    <dependencies>
        <dependency>
            <groupId>mysql</groupId>
            <artifactId>mysql-connector-java</artifactId>
            <!-- 版本要和你的 MySQl 依赖版本一致 -->
            <version>8.0.22</version>
        </dependency>
    </dependencies>
</plugin>
```



在 `generator-config.xml` 中配置必要的信息

```xml
<jdbcConnection driverClass="com.mysql.cj.jdbc.Driver"
                connectionURL="jdbc:mysql://localhost:3306/wiki?serverTimezone=UTC+8"
                userId="demo"
                password="123456">
</jdbcConnection>

<!-- pojo 类的位置 -->
<!-- 数据库表中每一行数据的实体类 Entity/Bean/POJ0 -->
<javaModelGenerator targetProject="src\main\java"
                    targetPackage="nju.gist.demo.POJO"/>

<!-- 生成的 mapper xml 的位置 -->
<sqlMapGenerator targetProject="src\main\resources"
                 targetPackage="mapper"/>

<!-- 生成的 mapper 类的位置 -->
<!-- mapper 就类似于数据库表 -->
<javaClientGenerator targetProject="src\main\java"
                     targetPackage="nju.gist.demo.mapper"
                     type="XMLMAPPER"/>

<!-- tableName 是数据库表名 -->
<table tableName="admin"/>
```

