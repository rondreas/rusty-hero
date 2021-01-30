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

// Basically our 'imports'
use bindings::{
    windows::win32::menus_and_resources::HMENU,
    windows::win32::system_services::{
        HINSTANCE, GetModuleHandleA,        // get the Handle to Instance typedef and the method for getting it,
        CS_OWNDC, CS_HREDRAW, CS_VREDRAW,   // get the Window Class Styles,
        CW_USEDEFAULT,                      // default values for windows height, width and position
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,    // window styles,
        LRESULT,                            // signed result of message processing,
        WM_SIZE, WM_DESTROY, WM_CLOSE,      // window messages,
        WM_ACTIVATEAPP, WM_PAINT,
    },
    windows::win32::windows_and_messaging::{
        HWND, WNDCLASSA,                    // handle for window and window class,
        LPARAM, WPARAM,                     // typedefined pointers for additional messages for window procedure,
        DefWindowProcA,                     // Default Window Procedure ( callback function )
        RegisterClassA,
        CreateWindowExA,
        MSG, GetMessageA, DispatchMessageA, // Messsage loop for our window,
        DestroyWindow, PostQuitMessage,
    },
    windows::win32::gdi::{ // Graphical Device Interface?
        HDC, PAINTSTRUCT,                   // Handle to Device Context, struct with information for painting a window, 
        BeginPaint, EndPaint,               // methods for painting a window, 
        PatBlt,                             // blit operation,
    },
    windows::{
        Result, BOOL,                       // get windows types,
    },
};

extern "system" fn wndproc (window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as i32 {
            WM_SIZE => {
                println!("WM_SIZE");
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_CLOSE => {
                println!("WM_CLOSE");
                DestroyWindow(window);
                LRESULT(0)
            }
            WM_ACTIVATEAPP => {
                println!("WM_ACTIVATEAPP");
                LRESULT(0)
            }
            WM_PAINT => {
                println!("WM_PAINT");

                let mut paint = PAINTSTRUCT::default();

                let device_context = BeginPaint(window, &mut paint);
                std::debug_assert!(device_context != HDC(0));

                let x = paint.rc_paint.left;
                let y = paint.rc_paint.top;

                let width = paint.rc_paint.right - paint.rc_paint.left;
                let height = paint.rc_paint.bottom - paint.rc_paint.top;

                // can't for the life of me find WHITENESS in the windows-rs but this
                // https://docs.microsoft.com/en-us/windows/win32/gdi/ternary-raster-operations
                // seem to show all the values at least ...
                let blit_success = PatBlt(device_context, x, y, width, height, 0x00ff0062);
                std::debug_assert!(blit_success != BOOL(0));

                EndPaint(window, &mut paint);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

fn main() -> Result<()> {

    unsafe { 

        // Let's get the hInstance to set up our window
        let instance = HINSTANCE(GetModuleHandleA(std::ptr::null()));
        std::debug_assert!(instance.0 != 0);

        // Let this be mutable, so we can edit properties down the line,
        let mut window = WNDCLASSA::default();

        window.h_instance = instance;

        // Set the window class name, 
        let window_class_name = b"HandmadeHeroWindowClass\0";
        window.lpsz_class_name = window_class_name.as_ptr() as *mut u8 as *mut i8;

        window.style = ( CS_OWNDC | CS_HREDRAW | CS_VREDRAW ) as u32;
        window.lpfn_wnd_proc = Some(wndproc);
        // window.hIcon = ; // @todo: in the future we might want to add an icon here,

        // Register the window class, return 0 if not successfull
        let atom = RegisterClassA(&window);
        std::debug_assert!(atom != 0);

        // The Title for our window,
        let title = b"Handmade Hero\0";

        CreateWindowExA(
            0,
            window_class_name.as_ptr() as *const i8,
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
