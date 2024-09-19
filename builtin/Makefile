.PHONY: all clean qcat lls qfind qgrep rdu

MAKE_BUILTIN = @cd $@/ && cargo build --release && cp target/release/$@ ../..

all: qcat lls qfind qgrep rdu

qcat:
	${MAKE_BUILTIN}

lls:
	${MAKE_BUILTIN}

clean:
	rm -rdf qcat/target/
	rm -rdf lls/target/
	rm -rdf qfind/target/
	rm -rdf qgrep/target/
	rm -rdf rdu/target/


