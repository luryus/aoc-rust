use std::{
    fmt::Display,
    num::TryFromIntError,
    ops::{Add, Mul, Neg, Sub},
};

use num_integer::Integer;
use num_traits::{ConstZero, Signed, Unsigned};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord2<T: Integer> {
    pub y: T,
    pub x: T,
}

impl<T: Display + Integer> Display for Coord2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(y={}, x={})", self.y, self.x)
    }
}

impl<T: Integer> From<(T, T)> for Coord2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

impl<T: Integer> From<Coord2<T>> for (T, T) {
    fn from(value: Coord2<T>) -> Self {
        (value.y, value.x)
    }
}

impl<T: Integer + Signed> Coord2<T> {
    fn try_into_unsigned_tuple<U: Integer + Unsigned + TryFrom<T, Error = TryFromIntError>>(
        self,
    ) -> Result<(U, U), TryFromIntError> {
        Ok((self.y.try_into()?, self.x.try_into()?))
    }
}

impl<T: Integer + Signed> Sub for Coord2<T> {
    type Output = Coord2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2 {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl<T: Integer> Add for Coord2<T> {
    type Output = Coord2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Coord2 {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl<T: Integer + Copy> Mul<T> for Coord2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Coord2 {
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }
}

impl<T: Integer + Signed> Neg for Coord2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}

impl<T: Integer + CheckedSignedSub> Coord2<T> {
    pub fn signed_sub(self, rhs: Self) -> Coord2<<T as CheckedSignedSub>::S> {
        Coord2 {
            y: self.y.checked_signed_sub(rhs.y).expect("Number out of range"),
            x: self.x.checked_signed_sub(rhs.x).expect("Number out of range")
        }
    }
}

impl<T> Coord2<T>
where
    T: Integer + ConstZero + CheckedAddSigned + Copy,
{
    pub fn checked_add_with_upper<B: Unsigned + Integer + TryFrom<T>>(
        self,
        rhs: Coord2<<T as CheckedAddSigned>::S>,
        bounds: (B, B),
    ) -> Option<Self> {
        let y = self.y.checked_add_signed(rhs.y)?;
        let x = self.x.checked_add_signed(rhs.x)?;
        (y >= T::ZERO
            && x >= T::ZERO
            && {
                let by: B = y.try_into().ok()?;
                by < bounds.0
            }
            && {
                let bx: B = x.try_into().ok()?;
                bx < bounds.1
            })
        .then_some(Coord2 { y, x })
    }
}

impl<T: Integer + ConstZero + Signed> Coord2<T> {
    pub fn checked_add_with_bounds(self, rhs: Self, lower: (T, T), upper: (T, T)) -> Option<Self> {
        let r = self + rhs;
        (r.y >= lower.0 && r.x >= lower.1 && r.y < upper.0 && r.x < upper.1).then_some(r)
    }
}

impl Coord2<usize> {
    pub fn usizes(self) -> (usize, usize) {
        self.into()
    }
}

impl Coord2<isize> {
    pub fn usizes(self) -> (usize, usize) {
        self.try_into_unsigned_tuple().unwrap()
    }
}

impl<T: Integer + ConstZero> Coord2<T> {
    pub const ZERO: Coord2<T> = Coord2{ y: T::ZERO, x: T::ZERO };
}

pub trait CheckedAddSigned {
    type S: Integer;

    fn checked_add_signed(self, other: Self::S) -> Option<Self>
    where
        Self: Sized;
}

impl CheckedAddSigned for usize {
    type S = isize;
    fn checked_add_signed(self, other: isize) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_signed(other)
    }
}

impl CheckedAddSigned for isize {
    type S = isize;
    fn checked_add_signed(self, other: isize) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add(other)
    }
}

pub trait CheckedSignedSub {
    type S: Integer + Signed;
    fn checked_signed_sub(self, other: Self) -> Option<Self::S> where Self: Sized;
}

impl CheckedSignedSub for usize {
    type S = isize;
    fn checked_signed_sub(self, rhs: Self) -> Option<isize> where Self: Sized {
        // https://github.com/rust-lang/rust/pull/126042
        let res = self.wrapping_sub(rhs) as isize;
        let overflow = (self >= rhs) == (res < 0);

        if !overflow {
            Some(res)
        } else {
            None
        }
    }
}