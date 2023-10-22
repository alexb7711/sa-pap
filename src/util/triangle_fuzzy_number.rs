//===============================================================================
// Import standard library
use core::ops::Add;
use std::ops;

//===============================================================================
// Import modules

//==============================================================================
/// The `TriangleFuzzyNumber` structure encapsulates the triangular fuzzy number
/// datatype.
//
#[derive(Debug)]
pub struct TriangleFuzzyNumber<T> {
    a: T,
    b: T,
    c: T,
}

//===============================================================================
/// Implementation of `triangle_fuzzy_number`.
//
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Sub
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + Into<f32>,
    > TriangleFuzzyNumber<T>
{
    //---------------------------------------------------------------------------
    /// Initialize the triangular fuzzy number object
    ///
    /// # Input
    /// * `a` : First variable of triangular fuzzy number
    /// * `b` : Second variable of triangular fuzzy number
    /// * `c` : Third variable of triangular fuzzy number
    ///
    /// # Output
    /// * `TriangleFuzzyNumber`: Triangular fuzzy number object
    ///
    pub fn new(a: T, b: T, c: T) -> TriangleFuzzyNumber<T> {
        return TriangleFuzzyNumber { a, b, c };
    }

    //---------------------------------------------------------------------------
    /// Initialize the triangular fuzzy number object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `T`: Value of the ranking function
    ///
    pub fn ranking_function(self: TriangleFuzzyNumber<T>) -> f32
    where
        <T as Add>::Output: Add<T>,
    {
        return (self.a.into() + self.b.into() + self.c.into()) / 3.0;
    }
}

//===============================================================================
/// Implementation of overloaded Copy/Clone for `triangle_fuzzy_number`.
//

//-------------------------------------------------------------------------------
/// Implementation of overloaded copy trait for `triangle_fuzzy_number`.
//
impl<T: Copy> Copy for TriangleFuzzyNumber<T> {}

//-------------------------------------------------------------------------------
/// Implementation of overloaded clone trait for `triangle_fuzzy_number`.
//
impl<T: Clone> Clone for TriangleFuzzyNumber<T> {
    fn clone(&self) -> TriangleFuzzyNumber<T> {
        return TriangleFuzzyNumber {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
        };
    }
}

//===============================================================================
/// Implementation of overloaded operators for `triangle_fuzzy_number`.
//

//-------------------------------------------------------------------------------
/// Implementation of overloaded addition for `triangle_fuzzy_number`.
//
impl<T: ops::Add<Output = T>> ops::Add<TriangleFuzzyNumber<T>> for TriangleFuzzyNumber<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded add assign for `triangle_fuzzy_number`.
//
impl<T: Copy + ops::Add<Output = T>> ops::AddAssign<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded subtraction operator for `triangle_fuzzy_number`.
//
impl<T: ops::Sub<Output = T>> ops::Sub<TriangleFuzzyNumber<T>> for TriangleFuzzyNumber<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded subtract assign for `triangle_fuzzy_number`.
//
impl<T: Copy + ops::Sub<Output = T>> ops::SubAssign<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded subtraction operator for `triangle_fuzzy_number`.
//
impl<T: ops::Mul<Output = T>> ops::Mul<TriangleFuzzyNumber<T>> for TriangleFuzzyNumber<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Self {
            a: self.a * other.a,
            b: self.b * other.b,
            c: self.c * other.c,
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded multiply assign for `triangle_fuzzy_number`.
//
impl<T: Copy + ops::Mul<Output = T>> ops::MulAssign<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a * other.a,
            b: self.b * other.b,
            c: self.c * other.c,
        };
    }
}

//===============================================================================
/// Implementation of overloaded indexing for `triangle_fuzzy_number`.
//

//-------------------------------------------------------------------------------
/// Implementation of overloaded indexing operator for `triangle_fuzzy_number`.
//
impl<T> ops::Index<usize> for TriangleFuzzyNumber<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            _ => panic!("Index out of bound"),
        };
    }
}

//-------------------------------------------------------------------------------
/// Implementation of overloaded mutable indexing operator for
/// `triangle_fuzzy_number`.
//
impl<T> ops::IndexMut<usize> for TriangleFuzzyNumber<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        return match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            _ => panic!("Index out of bound"),
        };
    }
}

//===============================================================================
/// Implementation of overloaded partial equality `triangle_fuzzy_number`.
//

//-------------------------------------------------------------------------------
/// Implementation of the equality operator for `triangular_fuzzy_number`.
//
impl<T: std::cmp::PartialEq> PartialEq for TriangleFuzzyNumber<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a && self.b == other.b && self.c == other.c;
    }
}

//-------------------------------------------------------------------------------
// Implementation of the equality operator for `triangular_fuzzy_number`.
//
// impl<T: std::cmp::PartialEq> PartialEq for TriangleFuzzyNumber<T> {
//     fn eq(&self, other: &Self) -> bool {
//         return self.a == other.b && self.b == other.b && self.c == other.c;
//     }
// }
