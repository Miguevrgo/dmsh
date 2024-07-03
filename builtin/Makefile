.PHONY: all clean qcat lls

CARGO_BUILD_DIR := target/release

all: qcat lls

qcat:
	cd $@/ && cargo build --release && cp target/release/$@ ../..

lls:
	cd $@/ && cargo build --release && cp target/release/$@ ../..

clean:
	rm -rdf qcat/target/
	rm -rdf lls/target/


