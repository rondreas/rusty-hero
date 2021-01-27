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
    windows::win32::menus_and_resources::HMENU,
    windows::win32::system_services::{
        HINSTANCE, GetModuleHandleA,        // get the Handle to Instance typedef and the method for getting it,
        CS_OWNDC, CS_HREDRAW, CS_VREDRAW,   // get the Window Class Styles,
        CW_USEDEFAULT,                      // default values for windows height, width and position
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,    // window styles,
        LRESULT,                            // signed result of message processing,
        WM_SIZE, WM_DESTROY, WM_CLOSE,      // enums for window notifications,
        WM_ACTIVATEAPP
    },
    windows::win32::windows_and_messaging::{
        HWND, WNDCLASSA,                    // handle for window and window class,
        LPARAM, WPARAM,                     // typedefined pointers for additional messages for window procedure,
        DefWindowProcA,                     // Default Window Procedure ( callback function )
        RegisterClassA,
        CreateWindowExA,
        MSG, GetMessageA, DispatchMessageA, // Messsage loop for our window,
        DestroyWindow, PostQuitMessage
    },
};

extern "system" fn wndproc (window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as i32 {
            WM_SIZE => {
                println!("Changed size\n");
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("Destroying window\n");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_CLOSE => {
                println!("Closing window\n");
                DestroyWindow(window);
                LRESULT(0)
            }
            WM_ACTIVATEAPP => {
                println!("Activated app\n");
                LRESULT(0)
            }
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

        let atom = RegisterClassA(&window);
        std::debug_assert!(atom != 0);

        let title = b"Handmade Hero\0";

        CreateWindowExA(
            0,
            window_class_name.as_ptr() as *mut i8,
            title.as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND(0),
            HMENU(0),
            instance,
            std::ptr::null_mut(), // LPVOID, long pointer void
        );

        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&mut message);
        }

        Ok(())
    }
}
