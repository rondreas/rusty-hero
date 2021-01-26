fn main() {
    windows::build!(
        windows::win32::system_services::{GetModuleHandleA, MB_OK, MB_ICONINFORMATION}
        windows::win32::windows_and_messaging::{MessageBoxA, WNDCLASSA}
    );
}