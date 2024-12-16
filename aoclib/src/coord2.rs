use std::{
    fmt::{Debug, Display},
    num::TryFromIntError,
    ops::{Add, Mul, Neg, Sub},
};

use ndarray::{Dim, NdIndex};
use num_integer::Integer;
use num_traits::{ConstZero, Signed, Unsigned, Zero};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord2<T: Integer = usize> {
    pub y: T,
    pub x: T,
}

impl<T: Ord + Integer> PartialOrd for Coord2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord + Integer> Ord for Coord2<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
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

unsafe impl NdIndex<Dim<[usize; 2]>> for Coord2<usize> {
    fn index_checked(&self, dim: &Dim<[usize; 2]>, strides: &Dim<[usize; 2]>) -> Option<isize> {
        self.usizes().index_checked(dim, strides)
    }

    fn index_unchecked(&self, strides: &Dim<[usize; 2]>) -> isize {
        self.usizes().index_unchecked(strides)
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
            y: self
                .y
                .checked_signed_sub(rhs.y)
                .expect("Number out of range"),
            x: self
                .x
                .checked_signed_sub(rhs.x)
                .expect("Number out of range"),
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

    pub fn wrapping_add<B>(self, rhs: Coord2<<T as CheckedAddSigned>::S>, bounds: (B, B)) -> Self
    where
        B: Integer + Unsigned + Copy + Debug + Into<T> + TryInto<<T as CheckedAddSigned>::S, Error: Debug>,
    {
        let by = bounds.0.try_into().expect("Can't convert unsigned bound to signed");
        let bx = bounds.1.try_into().expect("Can't convert unsigned bound to signed");

        let dy = rhs.y.mod_floor(&by);
        let dx = rhs.x.mod_floor(&bx);
        
        debug_assert!(dy >= <T as CheckedAddSigned>::S::zero());
        debug_assert!(dx >= <T as CheckedAddSigned>::S::zero());

        let y = self.y.checked_add_signed(dy).unwrap() % bounds.0.into();
        let x = self.x.checked_add_signed(dx).unwrap() % bounds.1.into();

        Coord2 { y, x }
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

    pub const UP: Self = Coord2 { y: -1, x: 0 };
    pub const DOWN: Self = Coord2 { y: 1, x: 0 };
    pub const LEFT: Self = Coord2 { y: 0, x: -1 };
    pub const RIGHT: Self = Coord2 { y: 0, x: 1 };
}

impl<T: Integer + ConstZero> Coord2<T> {
    pub const ZERO: Coord2<T> = Coord2 {
        y: T::ZERO,
        x: T::ZERO,
    };
}

pub trait CheckedAddSigned {
    type S: Integer + Copy;

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
    fn checked_signed_sub(self, other: Self) -> Option<Self::S>
    where
        Self: Sized;
}

impl CheckedSignedSub for usize {
    type S = isize;
    fn checked_signed_sub(self, rhs: Self) -> Option<isize>
    where
        Self: Sized,
    {
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
