use lazy_static::lazy_static;
//use std::sync::Mutex;
use std::sync::RwLock;

use winapi::{
    um::{
/*        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, IDI_APPLICATION, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, WM_LBUTTONUP, WM_KEYDOWN,
                  PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW},*/
                  winuser::*,       //  とりあえず全部。後で整理
        wingdi::{GetStockObject, WHITE_BRUSH, TextOutW,},
    },
    shared::{
        ntdef::*,
        windef::{HWND, HDC, HBRUSH, RECT, HMENU},
        minwindef::{HINSTANCE, UINT, WPARAM, LPARAM, LRESULT},
    },
    ctypes::{c_void},
};
use std::ptr;
use std::mem;
use std::collections::HashMap;

lazy_static! {
//    pub static ref H_INSTANCE: RwLock<HINSTANCE> = RwLock::new(ptr::null_mut());
    //  HINSTANCEのポインタアドレスをusizeで保持
    pub static ref H_INSTANCE_VALUE: RwLock<usize> = {
        RwLock::new(0usize)
    };

    //  MDIクライアントのハンドルのポインタアドレスをusizeで保持
    pub static ref HWND_MDICLIENT: RwLock<usize> = {
        RwLock::new(0usize)
    };
}

//  HINSTANCE取得関数
pub fn get_hinstance() -> HINSTANCE{
    let sss: usize = *(H_INSTANCE_VALUE.read().unwrap());
    sss as HINSTANCE
}

//  MDIクライアントのHWND取得関数
pub fn get_mdiclient_hwnd() -> HWND{
    let sss: usize = *(HWND_MDICLIENT.read().unwrap());
    sss as HWND
}


//  MDIクライアント用の構造体
pub struct CLIENTCREATESTRUCT
{
    hWindowMenu: HANDLE, 
    idFirstChild: UINT, 
}



//  入力イベント型エイリアス
type EventInput = fn(i32, i32, i32);

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//  入力イベント処理構造体
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
pub struct OnInputEvent
{
    down: EventInput,
    up: EventInput,
    click: EventInput,
}

impl OnInputEvent
{
    pub fn new() -> OnInputEvent{
        OnInputEvent{
            down: OnInputEvent::empty_func,
            up: OnInputEvent::empty_func,
            click: OnInputEvent::empty_func,
        }
    }
    fn empty_func(_event_type: i32, _x: i32, _y: i32){
    }

    pub fn set_events(&mut self, evt_donw: EventInput, evt_up: EventInput, evt_click: EventInput)
    {
        self.down = evt_donw;
        self.up = evt_up;
        self.click = evt_click;
    }

    pub fn set_down_event(&mut self, evt_donw: EventInput){ self.down = evt_donw; }
    pub fn set_up_event(&mut self, evt_up: EventInput){ self.up = evt_up; }
    pub fn set_click_event(&mut self, evt_click: EventInput){ self.click = evt_click; }
}


//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//  ウィンドウ情報構造体
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
pub struct WindowInfo{
    handle: HWND,
    class_name: String,
    width: i32,
    height: i32,
}
use std::marker::Sync;
use std::marker::Send;
unsafe impl Sync for WindowInfo{
}
unsafe impl Send for WindowInfo{
}
impl WindowInfo
{
    pub fn new() -> WindowInfo{
        WindowInfo{
            handle: ptr::null_mut(),
            class_name: String::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get_handle(&self) -> &HWND{
        &self.handle
    }

    pub fn set_class_name(&mut self, name: &str){
        self.class_name = name.to_string();
    }
    pub fn get_class_name(&self) -> &String{
        &self.class_name
    }

    pub fn set_size(&mut self, w: i32, h: i32){
        self.width = w;
        self.height = h;
    }
    pub fn get_width(&self) -> i32{
        self.width
    }
    pub fn get_height(&self) -> i32{
        self.height
    }

}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//  ウィンドウ処理構造体
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
use rand::Rng;

pub struct WindowProc
{
    w_info: WindowInfo,
    on_input: OnInputEvent
}

impl WindowProc
{
    pub fn new(width: i32, height:i32) -> WindowProc
    {
        let mut win: WindowProc = WindowProc{
            w_info: WindowInfo::new(),
            on_input: OnInputEvent::new()
        };
        win.w_info.set_size(width, height);
        let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
        let r: i32 = rng.gen();
        win.w_info.set_class_name(&r.to_string());
        win.create();
        win
    }

    pub fn set_on_inputdown(&mut self, fnc: EventInput){ self.on_input.set_down_event(fnc); }
    pub fn set_on_inputup(&mut self, fnc: EventInput){ self.on_input.set_up_event(fnc); }
    pub fn set_on_inputclick(&mut self, fnc: EventInput){ self.on_input.set_click_event(fnc); }

    pub fn create(&mut self) -> bool
    {
        if self.init_wininfo() == false{ return false; }

        let cls_name = WindowProc::encode(&self.w_info.class_name);

        unsafe
        {
            if !self.register_wndclass(&cls_name) {
                return false;
            }

            self.w_info.handle = self.create_window(&cls_name);
            if self.w_info.handle.is_null() {
                return false;
            }
        }
        true
    }

    pub fn show(&self){
        unsafe{
            ShowWindow(self.w_info.handle, SW_NORMAL);
        }
    }

    pub fn update(&self){
        unsafe{
            UpdateWindow(self.w_info.handle);
        }
    }

    pub fn run(&self)
    {
        unsafe
        {
            //  let mut msg = mem::uninitialized::<MSG>();
            let mut msg = mem::MaybeUninit::<MSG>::uninit().assume_init();
            loop {
                if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                    return;
                }
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }
    }

    pub fn run_once(&self)
    {
        unsafe
        {
            //  let mut msg = mem::uninitialized::<MSG>();
            let mut msg = mem::MaybeUninit::<MSG>::uninit().assume_init();
            if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                return;
            }
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }


    fn init_wininfo(&mut self) -> bool
    {
        //  設定済み
        if self.w_info.handle != ptr::null_mut(){
            return false;
        }
        self.w_info.width = if self.w_info.width <= 0{200} else{self.w_info.width};
        self.w_info.height = if self.w_info.height <= 0{200} else{self.w_info.height};
        if self.w_info.class_name.is_empty(){
            self.w_info.class_name = String::from("my_window_class_name");
        }
        true
    }

    unsafe fn register_wndclass(&self, class_name: &[u16]) -> bool {
        let mut winc = mem::zeroed::<WNDCLASSW>();
        winc.style = CS_HREDRAW | CS_VREDRAW;
        winc.lpfnWndProc = Some(WindowProc::win_proc);
        winc.hIcon = LoadIconW(ptr::null_mut(), IDI_APPLICATION);
        winc.hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
        winc.hbrBackground = GetStockObject(WHITE_BRUSH as i32) as HBRUSH;
        winc.lpszClassName = class_name.as_ptr();

        RegisterClassW(&winc) > 0
    }

    unsafe fn create_window(&self, class_name: &[u16]) -> HWND
    {
        let dispx = GetSystemMetrics(SM_CXSCREEN) / 2 - self.w_info.width / 2;
        let dispy = GetSystemMetrics(SM_CYSCREEN) / 2 - self.w_info.height / 2;

        CreateWindowExW(
            0,
            class_name.as_ptr(),
            WindowProc::encode("Hello, World!").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            dispx, dispy, self.w_info.width, self.w_info.height,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    }

    fn encode(source: &str) -> Vec<u16> {
        source.encode_utf16().chain(Some(0)).collect()
    }

    //  PAINTSTRUCT取得
    //  rustでは全メンバを初期化しないといけないので関数化
    fn new_paint_struct() -> PAINTSTRUCT
    {
        PAINTSTRUCT{
            hdc: ptr::null_mut(),
            fErase: FALSE as i32,
            rcPaint: RECT{ left: 0, top: 0, right: 0, bottom: 0 },
            fRestore: FALSE as i32,
            fIncUpdate: FALSE as i32,
            rgbReserved: [0; 32],
        }
    }

    unsafe extern "system" fn win_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT
    {
        let hdc: HDC;
        let mut ccsClient: CLIENTCREATESTRUCT = CLIENTCREATESTRUCT{
            hWindowMenu: ptr::null_mut(),
            idFirstChild: 0,
        };

        match msg {
            WM_CREATE => {
            //    let id = WinManager::get_id_from_hwnd(hwnd);
            //    if id >= 0{
            //        let asd = 0;
            //    }
            
                ccsClient.hWindowMenu = ptr::null_mut();
                ccsClient.idFirstChild = 50000;
                let ccsPtr = &mut ccsClient as *mut CLIENTCREATESTRUCT;
                let hmenu: HMENU = ptr::null_mut();
                let hClientWindow = CreateWindowExW(WS_EX_CLIENTEDGE, WindowProc::encode("MDICLIENT").as_ptr() , ptr::null_mut(),
                    WS_CHILD | WS_CLIPCHILDREN | WS_VISIBLE,
                    0 , 0 , 0 , 0 , hwnd , hmenu, get_hinstance() , ccsPtr as *mut c_void
                );
                //  MDIクライアントのHWNDを登録
                *(HWND_MDICLIENT.write().unwrap()) = hClientWindow as usize;
                MoveWindow(hClientWindow, 100, 100, 200, 200, 1);
                return 0;           
                
            //    return DefWindowProcW(hwnd, msg, w_param, l_param);
            },
            WM_DESTROY => PostQuitMessage(0),
            WM_LBUTTONUP => {
                hdc = GetDC(hwnd);
                TextOutW(hdc , 10 , 10 , WindowProc::encode("あいうえお").as_ptr(), 5);
                ReleaseDC(hwnd , hdc);
                println!("{}, {}", (l_param & 0xFFFF), ((l_param>>16) & 0xFFFF));
            },
            WM_KEYDOWN => {
                println!("{}, {}", w_param, l_param);
            },
            WM_PAINT => {
                let mut ps = WindowProc::new_paint_struct();
                hdc = BeginPaint(hwnd , &mut ps );
                TextOutW(hdc , 15 , 15 , WindowProc::encode("あいうえお").as_ptr(), 5);
                EndPaint(hwnd , &ps);
            },
        //    _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
            _ => return DefFrameProcW(hwnd , get_mdiclient_hwnd() , msg , w_param , l_param),
        };
        0
    }

}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//  ウィンドウ管理構造体
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
/*
lazy_static! {
    // 文法上の特徴: "static" ではなく "static ref" を使う
    static ref WIN_LIST: Mutex<HashMap<i32, WindowProc>> = Mutex::new(HashMap::new());
}
*/

pub struct WinManager
{
    win_list: HashMap<i32, WindowProc>,
}

impl WinManager
{
    pub fn new() -> WinManager
    {
        WinManager{
            win_list: HashMap::new()
        }
    }

    pub fn create(&mut self, width: i32, height: i32) -> i32
    {
        let id = self.win_list.len() as i32;
        self.win_list.insert(id, WindowProc::new(width, height));
        id
    }

    pub fn show(&self, id: i32) -> bool
    {
        if let Option::Some(win) = self.win_list.get(&id){
            win.show();
            return true;
        }
        false
    }

    pub fn update(&self, id: i32) -> bool
    {
        if let Option::Some(win) = self.win_list.get(&id){
            win.update();
            return true;
        }
        false
    }

    pub fn run(&self, id: i32) -> bool
    {
        if let Option::Some(win) = self.win_list.get(&id){
            win.run();
            return true;
        }
        false
    }

    pub fn get_id_from_hwnd(&self, hwnd: HWND) -> i32
    {
        for (key, value) in self.win_list.iter()
        {
            println!("{}", *key);
            if value.w_info.handle == hwnd{
                return *key;
            }
        }
        return -1;
    }

}



