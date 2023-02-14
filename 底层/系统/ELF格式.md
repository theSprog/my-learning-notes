### ELF 定义

64位 ELF 文件头定义如下

```c
// 位于 /usr/include/elf.h

#define EI_NIDENT (16)

typedef uint16_t Elf64_Half;	// 16 位
typedef uint32_t Elf64_Word;	// 32 位
typedef uint64_t Elf64_Addr;	// 地址
typedef uint64_t Elf64_Off;		// 偏移量

typedef struct
{
  unsigned char e_ident[EI_NIDENT];     /* Magic number and other info */
  Elf64_Half    e_type;                 /* Object file type */
  Elf64_Half    e_machine;              /* Architecture */
  Elf64_Word    e_version;              /* Object file version */
  Elf64_Addr    e_entry;                /* Entry point virtual address */
  Elf64_Off     e_phoff;                /* Program header table file offset */
  Elf64_Off     e_shoff;                /* Section header table file offset */
  Elf64_Word    e_flags;                /* Processor-specific flags */
  Elf64_Half    e_ehsize;               /* ELF header size in bytes */
  Elf64_Half    e_phentsize;            /* Program header table entry size */
  Elf64_Half    e_phnum;                /* Program header table entry count */
  Elf64_Half    e_shentsize;            /* Section header table entry size */
  Elf64_Half    e_shnum;                /* Section header table entry count */
  Elf64_Half    e_shstrndx;             /* Section header string table index */
} Elf64_Ehdr;
```







### 节头(section header)定义

















### 节(section)





















### 程序头(program header)



```c
#define PF_X            (1 << 0)        /* Segment is executable */
#define PF_W            (1 << 1)        /* Segment is writable */
#define PF_R            (1 << 2)        /* Segment is readable */

typedef struct
{
  Elf64_Word    p_type;                 /* Segment type */
  Elf64_Word    p_flags;                /* Segment flags */
  Elf64_Off     p_offset;               /* Segment file offset */
  Elf64_Addr    p_vaddr;                /* Segment virtual address */
  Elf64_Addr    p_paddr;                /* Segment physical address */
  Elf64_Xword   p_filesz;               /* Segment size in file */
  Elf64_Xword   p_memsz;                /* Segment size in memory */
  Elf64_Xword   p_align;                /* Segment alignment */
} Elf64_Phdr;
```

- p_type 字段表示该段的类型，如果值为PT_LOAD，则表示该段是一个可加载到内存中的段，

- p_offset表示程序表头记录相对于文件内的偏移。

- p_filesz表示程序表头记录所描述的数据长度，

- p_memsz表示对应数据加载到内存后的长度。

  > 通常情况下这两者相同，但由于加载到内存时可能需要字节对齐，因此后者有可能比前者要大
  >
  > 也就是说满足 p_memsz ≥ p_filesz ，否则内存无法容纳文件内容，该段不能完全加载。

- p_flags描述程序表头记录所描述数据的属性，如果取值 PF_X 表示描述的数据是可执行的代码，PF_W 表示所描述数据是可修改的数据，PF_R表示所描述数据具有可读性质

在程序头表中，所有 PT_LOAD 类型的程序头都**按照 p_vaddr 的值升序排列**