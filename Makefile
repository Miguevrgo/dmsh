CC := gcc
CFLAGS := -Wall -Wextra -Wpedantic -g

.PHONY: all
all: dmsh

dmsh: main.c
	${CC} ${CFLAGS} -o $@ $<

.PHONY: clean
clean:
	rm -f dmsh
