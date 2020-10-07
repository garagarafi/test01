use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

//extern crate number_traits;
//use number_traits::{Num, Sqrt};

//use ::num::traits::Num;

use super::math::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2D<T>
{
    pub x: T, pub y: T
}

//++++++++++++++++++++++++++++++++++++++++++++++++
//  
//++++++++++++++++++++++++++++++++++++++++++++++++
impl<T> Point2D<T>
{
    pub fn new(lx: T, ly: T) -> Self
    {
        Self{
            x: lx,
            y: ly
        }
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++
//  四則演算の結果がそれぞれのOutputを返す関数
//++++++++++++++++++++++++++++++++++++++++++++++++
impl<T> Point2D<T>
    where T: Add + Sub + Mul + Div + Copy + Clone + MyMath,
          <T as Mul>::Output:Add
          
{
    pub fn div(&self, d:T) -> Point2D<<T as Div>::Output> {
        Point2D{x:self.x/d,y:self.y/d}
    }

//    pub fn dot(&self, r: Point2D<T>) -> <<T as Mul>::Output as Add>::Output {
//        self.x * r.x + self.y * r.y
//    }

}

//++++++++++++++++++++++++++++++++++++++++++++++++
//  四則演算の結果がTを返す必要がある関数
//++++++++++++++++++++++++++++++++++++++++++++++++
impl<T> Point2D<T>
    where T: Add + Sub + Mul + Div + Copy + Clone + MyMath +
             Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T>      //  OutputがTということを明示。これで、関数の戻り値をTにできるらしい
{
    pub fn len(&self) -> T {
        (self.x * self.x + self.y * self.y).m_sqrt()
    }

    pub fn dot(&self, r: Point2D<T>) -> T {
        self.x * r.x + self.y * r.y
    }

    pub fn scale(&mut self, rate: T) {
        self.x = self.x * rate;
        self.y = self.y * rate;
    }
    pub fn scale_f(&mut self, rate: f64 ){
        let lx = self.x.m_to_f64();
        let ly = self.y.m_to_f64();
        self.x.m_from_f64(lx * rate);
        self.y.m_from_f64(ly * rate);
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++
//  四則足演算のオーバーロード
//++++++++++++++++++++++++++++++++++++++++++++++++
impl<T:Add> Add for Point2D<T>
{
    type Output = Point2D<<T as Add>::Output>;
    fn add(self, r: Point2D<T>) -> Point2D<<T as Add>::Output> {
        Point2D{x:self.x + r.x , y:self.y + r.y}
    }
}
//  こっちは関数の戻り値が"Point2D<T>"になる方
//impl<T:Add + Add<Output=T>> Add for Point2D<T>
//{
//    type Output = Point2D<<T as Add>::Output>;
//    fn add(self, r: Point2D<T>) -> Point2D<T> {
//        Point2D{x:self.x + r.x , y:self.y + r.y}
//    }
//}
impl<T:Sub> Sub for Point2D<T>
{
    type Output = Point2D<<T as Sub>::Output>;
    fn sub(self, r: Point2D<T>) -> Point2D<<T as Sub>::Output> {
        Point2D{x:self.x - r.x , y:self.y - r.y}
    }
}
impl<T:Mul> Mul for Point2D<T>
{
    type Output = Point2D<<T as Mul>::Output>;
    fn mul(self, r: Point2D<T>) -> Point2D<<T as Mul>::Output> {
        Point2D{x:self.x * r.x , y:self.y * r.y}
    }
}
impl<T:Div> Div for Point2D<T>
{
    type Output = Point2D<<T as Div>::Output>;
    fn div(self, r: Point2D<T>) -> Point2D<<T as Div>::Output> {
        Point2D{x:self.x / r.x , y:self.y / r.y}
    }
}

