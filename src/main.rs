use winapi::um::winuser::SendMessageA;
use winapi::shared::windef::HWND;


fn main() {
    unsafe {
    let hwnd_broadcast: HWND = 0xFFFF as HWND;
    SendMessageA(hwnd_broadcast, 0x0112, 0xF170, 2);
    }
}
