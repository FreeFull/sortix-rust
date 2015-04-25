all: hello

libs/libcore.rlib:
	mkdir -p libs
	rustc --target x86_64-unknown-sortix-gnu libcore/lib.rs -o libs/libcore.rlib

hello: libs/libcore.rlib hello.rs
	rustc --target x86_64-unknown-sortix-gnu -Llibs/ hello.rs -o hello
