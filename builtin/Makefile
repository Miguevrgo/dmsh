.PHONY: all
all: qcat

# .PHONY since we let Cargo do the work
.PHONY: qcat
qcat:
	cd $@/ && cargo build --release && cp target/release/$@ ../..

.PHONY: clean
clean:
	rm -rdf qcat/target/
