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
    Windows::Win32::Foundation::{PSTR, BOOL, HWND, LPARAM, WPARAM, LRESULT},
    Windows::Win32::System::LibraryLoader::GetModuleHandleA,
    Windows::Win32::UI::WindowsAndMessaging::{
        WNDCLASSA,                          // handle for window and window class,
        DefWindowProcA,                     // Default Window Procedure ( callback function )
        RegisterClassA,
        CreateWindowExA,
        MSG, GetMessageA, DispatchMessageA, // Messsage loop for our window,
        DestroyWindow, PostQuitMessage,
        CS_OWNDC, CS_HREDRAW, CS_VREDRAW,
        CW_USEDEFAULT,                      // default values for windows height, width and position
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,    // window styles,
        WM_SIZE, WM_DESTROY, WM_CLOSE,      // enums for window notifications,
        WM_ACTIVATEAPP, WM_PAINT,
    },
    Windows::Win32::Graphics::Gdi::{
        HDC, PAINTSTRUCT,
        BeginPaint, EndPaint, 
        PatBlt,
        WHITENESS,
    },
};

extern "system" fn wndproc (window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
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

                let x = paint.rcPaint.left;
                let y = paint.rcPaint.top;

                let width = paint.rcPaint.right - paint.rcPaint.left;
                let height = paint.rcPaint.bottom - paint.rcPaint.top;

                // can't for the life of me find WHITENESS in the windows-rs but this
                // https://docs.microsoft.com/en-us/windows/win32/gdi/ternary-raster-operations
                // seem to show all the values at least ...
                let blit_success = PatBlt(device_context, x, y, width, height, WHITENESS);
                std::debug_assert!(blit_success != BOOL(0));

                EndPaint(window, &mut paint);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

fn main() -> windows::Result<()> {

    unsafe { 

        // Let's get the hInstance to set up our window
        let instance = GetModuleHandleA(None);
        std::debug_assert!(instance.0 != 0);

        // Let this be mutable, so we can edit properties down the line,
        let mut window = WNDCLASSA::default();

        window.hInstance = instance;

        // Set the window class name, 
        let window_class = "HandmadeHeroWindowClass";
        window.lpszClassName = PSTR(b"HandmadeHeroWindowClass\0".as_ptr() as _);

        window.style = CS_OWNDC | CS_HREDRAW | CS_VREDRAW;
        window.lpfnWndProc = Some(wndproc);
        // window.hIcon = ; // @todo: in the future we might want to add an icon here,

        // Register the window class, return 0 if not successfull
        let atom = RegisterClassA(&window);
        std::debug_assert!(atom != 0);

        CreateWindowExA(
            Default::default(),
            window_class,
            "Handmade Hero",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
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
