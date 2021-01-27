/*
 * Attempting to follow along making Handmade Hero in Rust now that
 * windows ported their api to rust.
 * 
 * To include api calls add them first in the bindings/build.rs then to the use bindings
 * 
 * Thank god for others having solved what I'm struggling with :pray:
 * https://github.com/kennykerr/samples-rs/tree/master/create_window
 * 
 */

use bindings::{
    windows::win32::system_services::{
        HINSTANCE, GetModuleHandleA,        // get the Handle to Instance typedef and the method for getting it,
        CS_OWNDC, CS_HREDRAW, CS_VREDRAW,   // get the Window Class Styles,
        LRESULT,                            // signed result of message processing,
        WM_SIZE, WM_DESTROY, WM_CLOSE,      // enums for window notifications,
        WM_ACTIVATEAPP
    },
    windows::win32::windows_and_messaging::{
        HWND, WNDCLASSA,                    // Get structs for Window Handle and Window Class,
        LPARAM, WPARAM,                     // typedefined pointers for additional messages for window procedure,
        DefWindowProcA                      // Default Window Procedure ( callback function )
    },
};

extern "system" fn wndproc (window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as i32 {
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

fn main() -> windows::Result<()> {

    unsafe { 

        // Let's get the hInstance to set up our window
        let instance = HINSTANCE(GetModuleHandleA(0 as *const i8));

        // Let this be mutable, so we can edit properties down the line,
        let mut window = WNDCLASSA::default();

        window.style = ( CS_OWNDC | CS_HREDRAW | CS_VREDRAW ) as u32;
        window.lpfn_wnd_proc = Some(wndproc);
        window.h_instance = instance;
        // window.hIcon = ; // @todo: in the future we might want to add an icon here,

        // Set the window class name, 
        let window_class_name = b"HandmadeHeroWindowClass\0";
        window.lpsz_class_name = window_class_name.as_ptr() as *mut i8;

        Ok(())
    }
}
