//  アプリケーション設定

use super::super::types::points::{Point2D};

#[derive(Debug)]
pub struct ClAppSetting
{
    pub win_pos: Point2D<i32>,
    pub win_size: Point2D<i32>,
}

impl ClAppSetting
{
    //---------------------------------------------
    //  生成
    pub fn new() -> ClAppSetting
    {
        ClAppSetting{
            win_pos: Point2D::new(0, 0),
            win_size: Point2D::new(300, 200),
        }
    }

    //---------------------------------------------
    //  位置設定
    pub fn set_win_pos(&mut self, x: i32, y: i32){
        self.win_pos.x = x;
        self.win_pos.y = y;
    }

    pub fn get_win_pos_x(&self) -> i32 { self.win_pos.x }
    pub fn get_win_pos_y(&self) -> i32 { self.win_pos.y }

    //---------------------------------------------
    //  サイズ設定
    pub fn set_win_size(&mut self, width: i32, height: i32){
        self.win_size.x = width;
        self.win_size.y = height;
    }

    pub fn get_win_width(&self) -> i32 { self.win_size.x }
    pub fn get_win_height(&self) -> i32 { self.win_size.y }

}

