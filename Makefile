all: hello

libs:
	mkdir -p libs

libcore.rlib: libcore libs
	rustc --target x86_64-unknown-sortix-gnu libcore/lib.rs -o libs/libcore.rlib

hello: libcore.rlib hello.rs
	rustc --target x86_64-unknown-sortix-gnu -Llibs/ hello.rs -o hello
