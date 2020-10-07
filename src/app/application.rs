use winapi::{
    shared::{
        minwindef::{HINSTANCE},
    },
};

use super::super::gui::form;
use super::app_setting::{ClAppSetting};

pub struct ClApplication
{
    setting: ClAppSetting,
}

impl ClApplication
{
    pub fn new() -> ClApplication
    {
        ClApplication{
            setting: ClAppSetting::new(),
        }
    }

    pub fn run(&mut self) -> i32
    {
        self.initialize();

        let mut fm = form::Form::new(self.setting.get_win_width(), self.setting.get_win_height());
        fm.create();
    //  fm.run_once();
        fm.run();

        0
    }

    fn initialize(&mut self)
    {
        self.setting.set_win_pos(100, 50);
        self.setting.set_win_size(640, 480);
    }
}

