use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Field:
    Copy
    + Clone
    + Eq
    + core::fmt::Debug
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
{
    // 加法単位元
    fn zero() -> Self;
    // 乗法単位元
    fn one() -> Self;
    fn is_zero(self) -> bool {
        self == Self::zero()
    }

    // 乗法に関する逆元
    fn inv(self) -> Self;
}

#[derive(Copy, Clone)]
pub struct F2(u8);

impl Add for F2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 ^ rhs.0)
    }
}

impl F2 {
    fn new(x: u8) -> Self {
        F2(x)
    }
}
