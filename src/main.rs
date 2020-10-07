// main ではなく WinMain をエントリーポイントにする
#![no_main]

use winapi::{
  shared::{
      minwindef::{HINSTANCE, UINT, WPARAM, LPARAM, LRESULT},
    },
};

use hello_cargo::pos;   //←プロジェクト名::module（プロジェクト名→クレートになる？）

//extern "C" {
//    fn abs(x: i32) -> i32;		// C言語のabc()ライブラリを定義
//    fn cos(x: f64) -> f64;		// C言語のabc()ライブラリを定義
//}

//use hello_cargo::win32::dialog;
use hello_cargo::win32::window;   //  HINSTANCEのアドレスをstaticで保存するのに使う

//use hello_cargo::gui::form;


use std::os::raw::{c_char, c_void, c_int};

use hello_cargo::app::application;


// 関数の命名規則がスネークケースじゃない指定
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(hinst : HINSTANCE, _ : *const c_void, _ : *const c_char, _ : c_int) -> c_int
{
  *(window::H_INSTANCE_VALUE.write().unwrap()) = hinst as usize;
  let mut app = application::ClApplication::new();
  let _res = app.run();

    return 0
}

fn main()
{
//    let mut app = application::ClApplication::new();
//    let _res = app.run();


//  unsafe {
//     println!("{}", abs(-123));
//     println!("{}", cos(0.0));
//  }

/*
  let mut w_info = window::WindowInfo::new();
  w_info.set_size(640, 480);
  let mut main_win = window::WindowProc::new(640, 480);
//  main_win.create(&mut w_info);

main_win.set_on_inputdown(|x:i32, y:i32, z:i32|{println!("ダウン！")});

  main_win.show();
  main_win.update();
  main_win.run();
*/
/*
let win_id = window::WinManager::create(640, 480);
window::WinManager::show(win_id);
window::WinManager::update(win_id);
window::WinManager::run(win_id);
*/

//  window::wmain();
/*
let mut fmw = form::Form::new();
fmw.create(10, 10);
fmw.run_once();

  let mut fm = form::Form::new();
  fm.create(10, 10);
  fm.run();
*/
//  dialog::open_okcancel("abcd", "efg");
  
  let bb = pos::funccc();
   println!("{}", bb);

   let mut pp = pos::Pos::new();

   let mut chara = pos::CharaInfo::new();

   let ppp = &mut pp;
   ppp.set_y(8.0);

   //pp.x = 16.0;
   pp.set_x(16.0);

   println!("{} {} {}", pp.get_x(), pp.get_y(), pp.get_z());

   chara.position.set_x(12.0);
   println!("{} {} {}", chara.position.get_x(), chara.position.get_y(), chara.position.get_z());

   println!("pause");

  //  println!("Hello, worlasdasd!");
}


