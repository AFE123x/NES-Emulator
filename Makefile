CC=clang

CFLAG= -fsanitize=address,undefined -Og -g

bin/main: obj/main.o
	$(CC) -o $@ -c $^ $(CFLAG)
obj/main.o: src/main.c
	$(CC) -o $@ $^ $(CFLAG)
