//use ::win32::dialog;
use super::super::win32::window;

use super::common::{CommonInfo, EGuiType};


pub struct Form
{
    common: CommonInfo,
    win_proc: window::WindowProc,
}

impl Form
{
    pub fn new(width: i32, height:i32) -> Form
    {
        Form{
            common: CommonInfo::new(EGuiType::WIN),
            win_proc: window::WindowProc::new(width, height)
        }
    }

    pub fn create(&mut self)
    {
        self.win_proc.show();
        self.win_proc.update();
    }

    pub fn run(&self)
    {
        self.win_proc.run();
    }
    pub fn run_once(&self)
    {
        self.win_proc.run_once();
    }
}
