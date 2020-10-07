use winapi::um::winuser::{MessageBoxW, MB_OK, MB_OKCANCEL};
use std::ptr;

pub struct Dialog
{

}

impl Dialog
{
    fn open(text: &str, caption: &str, utype: u32) -> i32
    {
        unsafe {
            return MessageBoxW(
                ptr::null_mut(), //hWnd: HWND
                Dialog::encode(text).as_ptr(), //lpText: LPCWSTR
                Dialog::encode(caption).as_ptr(), //lpCaption: LPCWSTR
                utype); //uType: UINT
        }
    }

    fn encode(source: &str) -> Vec<u16> {
        //  std::str::encode_utf16()で UTF16 へ変換。.chain(Some(0))で NULL 終端文字を追加。.collect()でVec<u16>にまとめ
        source.encode_utf16().chain(Some(0)).collect()
    }
}

pub fn open(text: &str, caption: &str)
{
    Dialog::open( text, caption, MB_OK );
}
pub fn open_okcancel(text: &str, caption: &str) -> i32
{
    return Dialog::open( text, caption, MB_OKCANCEL );
}
