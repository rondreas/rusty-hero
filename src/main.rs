/*
 * Attempting to follow along making Handmade Hero in Rust now that
 * windows ported their api to rust.
 * 
 * To include api calls add them first in the bindings/build.rs then to the use bindings
 */


use bindings::{
    windows::win32::system_services::{MB_OK, MB_ICONINFORMATION},
    windows::win32::windows_and_messaging::{MessageBoxA, HWND},
};

fn main() -> windows::Result<()> {

    unsafe {
        let title = b"Hello\0";
        let text = b"Hello, World!\0";

        MessageBoxA(
            HWND(0),
            text.as_ptr() as *const i8,
            title.as_ptr() as *const i8,
            MB_OK as u32 | MB_ICONINFORMATION as u32,
        );
    }

    Ok(())
}
