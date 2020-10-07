//  math
//  数値演算をジェネリクスで使うためのトレイト
//  数値のプリミティブに設定する

//use std::ops::Add;
//use std::ops::Sub;
//use std::ops::Mul;
//use std::ops::Div;

pub trait MyMath
{
   fn m_from_f64(&mut self, src: f64);
   fn m_to_f64(&self) -> f64;
   fn m_sqrt(&self) -> Self;
}

//-------------------------------------------------
//  f64
impl MyMath for f64
{
    fn m_from_f64(&mut self, src: f64){ *self = src; }
    fn m_to_f64(&self) -> f64{ *self }
    fn m_sqrt(&self) -> Self{ self.sqrt() }
}

//-------------------------------------------------
//  f32
impl MyMath for f32
{
    fn m_from_f64(&mut self, src: f64){ *self = src as f32; }
    fn m_to_f64(&self) -> f64{ *self as f64 }
    fn m_sqrt(&self) -> Self{ self.sqrt() }
}

//-------------------------------------------------
//  i32
impl MyMath for i32
{
    fn m_from_f64(&mut self, src: f64){ *self = src as i32; }
    fn m_to_f64(&self) -> f64{ *self as f64 }
    fn m_sqrt(&self) -> Self{ self.m_to_f64().sqrt() as i32 }
}
