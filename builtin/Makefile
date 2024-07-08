.PHONY: all clean qcat lls

MAKE_BUILTIN = @cd $@/ && cargo build --release && cp target/release/$@ ../..

all: qcat lls

qcat:
	${MAKE_BUILTIN}

lls:
	${MAKE_BUILTIN}

clean:
	rm -rdf qcat/target/
	rm -rdf lls/target/


