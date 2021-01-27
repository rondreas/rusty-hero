fn main() {
    windows::build!(
        windows::win32::menus_and_resources::HMENU
        windows::win32::system_services::{
            HINSTANCE, GetModuleHandleA,        // get the Handle to Instance typedef and the method for getting it,
            CS_OWNDC, CS_HREDRAW, CS_VREDRAW,   // get the Window Class Styles,
            CW_USEDEFAULT,                      // default values for windows height, width and position
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,    // window styles,
            LRESULT,                            // signed result of message processing,
            WM_SIZE, WM_DESTROY, WM_CLOSE,      // enums for window notifications,
            WM_ACTIVATEAPP
        }
        windows::win32::windows_and_messaging::{
            HWND, WNDCLASSA,                    // handle for window and window class,
            LPARAM, WPARAM,                     // typedefined pointers for additional messages for window procedure,
            DefWindowProcA,                     // Default Window Procedure ( callback function )
            RegisterClassA,
            CreateWindowExA,
            MSG, GetMessageA, DispatchMessageA, // Messsage loop for our window,
            DestroyWindow, PostQuitMessage
        }
    );
}