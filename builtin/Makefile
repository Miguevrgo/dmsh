.PHONY: all
all: fcat

# .PHONY since we let Cargo do the work
.PHONY: fcat
fcat:
	cd $@/ && cargo build && cp target/debug/$@ ../..

.PHONY: clean
clean:
	rm -rdf fcat/target/
