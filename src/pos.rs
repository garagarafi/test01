pub struct Pos
{
    x: f32, y: f32, z: f32, 
}
impl Pos
{
    //++++++++++++++++++++++++
    pub fn new() -> Pos{
        Pos{
            x:0.0, y:0.0, z:0.0
        }
    }
    pub fn get_x(&self) -> f32{ return self.x; }
    pub fn get_y(&self) -> f32{ return self.y; }
    pub fn get_z(&self) -> f32{ return self.z; }
    pub fn set_x(&mut self, lx:f32) -> f32{ self.x = lx; return self.x; }
    pub fn set_y(&mut self, ly:f32) -> f32{ self.y = ly; return self.y; }
    pub fn set_z(&mut self, lz:f32) -> f32{ self.z = lz; return self.z; }
    pub fn set(&mut self, lx:f32, ly:f32, lz:f32){
        self.x = lx;
        self.y = ly;
        self.z = lz;
    }
}

pub struct CharaInfo
{
    pub position: Pos,
    pub angle: Pos,
}
impl CharaInfo
{
    //++++++++++++++++++++++++
    pub fn new() -> CharaInfo{
        CharaInfo{
            position: Pos::new(),
            angle: Pos::new(),
        }
    }
}

pub fn funccc() -> i32{
5
}

