CC = gcc
CFLAG = -Wconversion -Wall -Werror -Og -g -fsanitize=address,undefined


bin/gb: obj/cpu.o obj/bus.o obj/main.o
	$(CC) -o $@ $^ $(CFLAG)
tests: bin/cputests
bin/cputests: src/cpu.c src/bus.c
	gcc -o $@ $^ -Wconversion -Wall -Werror -Og -g -lcriterion -DCPU_UNIT_TESTS  -fsanitize=address,undefined
obj/cpu.o: src/cpu.c
	$(CC) -o $@ -c $^ $(CFLAG)
obj/bus.o: src/bus.c
	$(CC) -o $@ -c $^ $(CFLAG)
obj/main.o: src/main.c
	$(CC) -o $@ -c $^ $(CFLAG)
clean: bin/gb
	rm -f obj/*.o bin/gb bin/cputests