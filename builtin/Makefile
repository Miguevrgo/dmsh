.PHONY: all
all: qcat

# .PHONY since we let Cargo do the work
.PHONY: qcat
qcat:
	cd $@/ && cargo build && cp target/debug/$@ ../..

.PHONY: clean
clean:
	rm -rdf qcat/target/
