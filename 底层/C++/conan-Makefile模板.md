conanfile.txt

```txt
[requires]
库名称/版本

[generators]
make
```



makefile

```makefile
#----------------------------------------
#     Prepare flags from make generator
#----------------------------------------

# conanbuildinfo.mak 即 conan 创建的makefile文件
include conanbuildinfo.mak

CFLAGS              += $(CONAN_CFLAGS)
CXXFLAGS            += $(CONAN_CXXFLAGS)
CPPFLAGS            += $(addprefix -I, $(CONAN_INCLUDE_DIRS))
CPPFLAGS            += $(addprefix -D, $(CONAN_DEFINES))
LDFLAGS             += $(addprefix -L, $(CONAN_LIB_DIRS))
LDLIBS              += $(addprefix -l, $(CONAN_LIBS))
EXELINKFLAGS        += $(CONAN_EXELINKFLAGS)

#----------------------------------------
#     Make variables for a sample App
#----------------------------------------

# 定义变量
SRCS          = main.cpp
OBJS          = main.o
EXE_FILENAME  = main

#----------------------------------------
#     Make Rules
#----------------------------------------

# 编译规则
.PHONY                  :   exe
exe                     :   $(EXE_FILENAME)

$(EXE_FILENAME)         :   $(OBJS)
	g++ $(OBJS) $(CXXFLAGS) $(LDFLAGS) $(LDLIBS) -o $(EXE_FILENAME)

%.o                     :   $(SRCS)
	g++ -c $(CPPFLAGS) $(CXXFLAGS) $< -o $@
```



bash

```shell
$ conan install <conanfile_folder>/
$ make exe
```

