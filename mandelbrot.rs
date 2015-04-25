#![feature(no_std,start,core,lang_items)]
#![no_main]
#![no_std]

extern crate core;
use core::prelude::*;

#[allow(dead_code)]
mod dispd {
    pub enum Session {}
    pub enum Window {}
    pub enum Framebuffer {}
    #[link(name = "dispd")]
    extern {
        pub fn dispd_initialize(argc: *mut i32, argv: *mut *mut *mut u8) -> bool;

        pub fn dispd_attach_default_session() -> *mut Session;
        pub fn dispd_detach_session(session: *mut Session) -> bool;

        pub fn dispd_session_setup_game_rgba(session: *mut Session) -> bool;

        pub fn dispd_create_window_game_rgba(session: *mut Session) -> *mut Window;
        pub fn dispd_destroy_window(window: *mut Window) -> bool;

        pub fn dispd_begin_render(window: *mut Window) -> *mut Framebuffer;
        pub fn dispd_finish_render(framebuffer: *mut Framebuffer) -> bool;

        pub fn dispd_get_framebuffer_data(framebuffer: *mut Framebuffer) -> *mut u8;
        pub fn dispd_get_framebuffer_pitch(framebuffer: *mut Framebuffer) -> usize;
        pub fn dispd_get_framebuffer_format(framebuffer: *mut Framebuffer) -> i32;
        pub fn dispd_get_framebuffer_height(framebuffer: *mut Framebuffer) -> i32;
        pub fn dispd_get_framebuffer_width(framebuffer: *mut Framebuffer) -> i32;
    }
}

extern {
    pub fn getchar() -> i32;
    pub fn exit(status: i32);
}

unsafe fn draw_mandelbrot(window: *mut dispd::Window) {
    use dispd::*;
    let fb = dispd_begin_render(window);
    if fb.is_null() {
        exit(1);
    }
    let xres = dispd_get_framebuffer_width(fb) as usize;
    let yres = dispd_get_framebuffer_height(fb) as usize;
    let bpp = dispd_get_framebuffer_format(fb) as usize;
    let pitch = dispd_get_framebuffer_pitch(fb);
    let linesize = pitch / (bpp / 8);
    let framesize = pitch * yres;
    let buf = dispd_get_framebuffer_data(fb) as *mut u32;
    if buf.is_null() {
        exit(1);
    }
    for y in 0..yres {
        let line = buf.offset((y*linesize) as isize);
        for x in 0..xres {
            let c = ((x as f32)/(xres as f32)*3.0 - 2.5,
                    (y as f32)/(yres as f32)*3.0 - 1.5);
            let mut z = (0.0, 0.0);
            let mut i = 0;
            while i < 3000 && z.0*z.0 + z.1*z.1 < 4.0 {
                z = (z.0*z.0 - z.1*z.1 + c.0, 2.0*z.0*z.1 + c.1);
                i += 1;
            }
            i = i << 8;
            *line.offset(x as isize) = i | 0xFF000000;
        }
    }
    dispd_finish_render(fb);
}

#[lang = "start"]
#[no_mangle]
pub extern fn main(mut _argc: i32, mut _argv: *const *const u8) -> i32 {
    unsafe {
        use dispd::*;
        if !dispd_initialize(&mut _argc, &mut _argv as *mut _ as *mut *mut *mut u8) {
            return 1;
        }

        let session = dispd_attach_default_session();
        if session.is_null() {
            return 1;
        }
        if !dispd_session_setup_game_rgba(session) {
            return 1;
        }
        let window = dispd_create_window_game_rgba(session);
        if window.is_null() {
            return 1;
        }

        draw_mandelbrot(window);

        getchar();

        dispd_destroy_window(window);
        dispd_detach_session(session);
    }
    0
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(args: core::fmt::Arguments,
                                file: &'static str, line: u32) -> !
{
    unsafe {
        exit(2);
    }
    loop {}
}
