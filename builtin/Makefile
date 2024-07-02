.PHONY: all clean

CARGO_BUILD_DIR := target/release

all: qcat lls

qcat:
	$(MAKE) -C $@ build
	cp $@/$(CARGO_BUILD_DIR)/qcat .

lls:
	$(MAKE) -C $@ build
	cp $@/$(CARGO_BUILD_DIR)/lls .

clean:
	rm -rdf qcat/target/
	rm -rdf lls/target/

.PHONY: qcat/build lls/build
qcat/build lls/build:
	cd $* && cargo build --release

