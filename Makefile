CC := gcc
CFLAGS := -Wall -Wextra -Wpedantic -g

.PHONY: all
all: dmsh builtin

dmsh: main.c
	${CC} ${CFLAGS} -o $@ $<

.PHONY: builtin
builtin:
	${MAKE} -C $@

.PHONY: clean
clean:
	@${MAKE} -C builtin $@
	rm -f dmsh fcat
