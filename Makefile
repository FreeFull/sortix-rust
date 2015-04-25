all: hello mandelbrot

clean: clean-libs clean-exe

clean-libs:
	rm libs/*

clean-exe:
	rm hello mandelbrot

libs/libcore.rlib:
	mkdir -p libs
	rustc --target x86_64-unknown-sortix-gnu libcore/lib.rs -o libs/libcore.rlib

hello: libs/libcore.rlib hello.rs
	rustc --target x86_64-unknown-sortix-gnu -Llibs/ hello.rs -o hello

mandelbrot: libs/libcore.rlib mandelbrot.rs
	rustc --target x86_64-unknown-sortix-gnu -Llibs/ mandelbrot.rs -o mandelbrot
