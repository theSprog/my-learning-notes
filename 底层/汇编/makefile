all: hello
	
hello.o: hello.asm
	nasm -g -f elf64 -o hello.o hello.asm -l hello.lst

hello: hello.o	
	ld -e _start2 -o hello hello.o