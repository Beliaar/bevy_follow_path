use bevy::math::Vec2;
use geo_nd::Vector;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default)]
pub(crate) struct Vec2Geo(pub(crate) Vec2);

impl Clone for Vec2Geo {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Vec2Geo {}

impl Debug for Vec2Geo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Vec2Geo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<[f32; 2]> for Vec2Geo {
    fn as_ref(&self) -> &[f32; 2] {
        self.0.as_ref()
    }
}

impl AsMut<[f32; 2]> for Vec2Geo {
    fn as_mut(&mut self) -> &mut [f32; 2] {
        self.0.as_mut()
    }
}

impl AsRef<[f32]> for Vec2Geo {
    fn as_ref(&self) -> &[f32] {
        self.0.as_ref()
    }
}

impl AsMut<[f32]> for Vec2Geo {
    fn as_mut(&mut self) -> &mut [f32] {
        self.0.as_mut()
    }
}

impl IndexMut<usize> for Vec2Geo {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Index<usize> for Vec2Geo {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0.x,
            1 => &self.0.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Neg for Vec2Geo {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Add<Self> for Vec2Geo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Add<f32> for Vec2Geo {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0.add(rhs))
    }
}

impl AddAssign<Self> for Vec2Geo {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.add(rhs.0)
    }
}

impl AddAssign<f32> for Vec2Geo {
    fn add_assign(&mut self, rhs: f32) {
        self.0 = self.0.add(rhs)
    }
}

impl Sub<Self> for Vec2Geo {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Sub<f32> for Vec2Geo {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0.sub(rhs))
    }
}

impl SubAssign<Self> for Vec2Geo {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.sub(rhs.0)
    }
}

impl SubAssign<f32> for Vec2Geo {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 = self.0.sub(rhs)
    }
}

impl Mul<Self> for Vec2Geo {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl Mul<f32> for Vec2Geo {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl MulAssign<Self> for Vec2Geo {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.mul(rhs.0)
    }
}

impl MulAssign<f32> for Vec2Geo {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = self.0.mul(rhs)
    }
}

impl Div<Self> for Vec2Geo {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

impl Div<f32> for Vec2Geo {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.div(rhs))
    }
}

impl DivAssign<Self> for Vec2Geo {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.0.div(rhs.0)
    }
}

impl DivAssign<f32> for Vec2Geo {
    fn div_assign(&mut self, rhs: f32) {
        self.0 = self.0.div(rhs)
    }
}

impl Vector<f32, 2> for Vec2Geo {
    fn from_array(data: [f32; 2]) -> Self {
        Self(Vec2::new(data[0], data[1]))
    }

    fn zero() -> Self {
        Self(Vec2::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == Vec2::ZERO
    }

    fn set_zero(&mut self) {
        self.0 = Vec2::ZERO
    }

    fn reduce_sum(&self) -> f32 {
        self.0.x + self.0.y
    }

    fn mix(&self, other: &Self, t: f32) -> Self {
        Self(self.0.lerp(other.0, t))
    }

    fn dot(&self, other: &Self) -> f32 {
        self.0.dot(other.0)
    }
}
