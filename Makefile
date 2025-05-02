all: build

build:
	CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo build --release 

install: build
	sudo install -m755 target/release/svr /bin/svr

uninstall:
	sudo rm -f /bin/svr

clean:
	cargo clean

.PHONY: all build install uninstall clean
