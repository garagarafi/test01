
use super::super::types::points::{Point2D};

pub enum EGuiType
{
    WIN,
    TEXT,
}

//  GUI系共通トレイト
pub trait GuiTrt
{
    fn create();
}

//  GUI系共通情報
pub struct CommonInfo
{
    gui_type: EGuiType,
    id: i32,
    pos: Point2D<i32>,
    size: Point2D<i32>,
}
impl CommonInfo
{
    pub fn new(ltype: EGuiType) -> CommonInfo
    {
        CommonInfo{
            gui_type: ltype,
            id: -1,
            pos: Point2D::new(0, 0),
            size: Point2D::new(50, 50),
        }
    }
}

