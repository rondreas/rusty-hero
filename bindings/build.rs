fn main() {
    windows::build!(
        Windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM, LRESULT},
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::UI::WindowsAndMessaging::{
            HMENU,
            WNDCLASSA,                    // handle for window and window class,
            DefWindowProcA,                     // Default Window Procedure ( callback function )
            RegisterClassA,
            CreateWindowExA,
            MSG, GetMessageA, DispatchMessageA, // Messsage loop for our window,
            DestroyWindow, PostQuitMessage,
            CW_USEDEFAULT,                      // default values for windows height, width and position
            WM_SIZE, WM_DESTROY, WM_CLOSE,      // enums for window notifications,
            WM_ACTIVATEAPP, WM_PAINT,
        },
        Windows::Win32::Graphics::Gdi::{
            HDC, PAINTSTRUCT,
            BeginPaint, EndPaint, 
            PatBlt,
        },
    );
}