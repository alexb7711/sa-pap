//===============================================================================
// Import standard library
use std::ops;

//===============================================================================
// Import modules

pub trait FuzzyNumberTrait {}

impl FuzzyNumberTrait for u32 {}
impl FuzzyNumberTrait for i32 {}
impl FuzzyNumberTrait for f32 {}

//==============================================================================
/// The `TriangleFuzzyNumber` structure encapsulates the triangular fuzzy number
/// datatype.
//
#[derive(Debug, PartialEq)]
pub struct TriangleFuzzyNumber<T: FuzzyNumberTrait> {
    a: T,
    b: T,
    c: T,
}

//==============================================================================
/// The `TriangleFuzzyNumberIntoIterator` structure encapsulates the triangular fuzzy number
/// iterator datatype.
//
pub struct TriangleFuzzyNumberIntoIterator<T: FuzzyNumberTrait> {
    tfn: TriangleFuzzyNumber<T>,
    index: usize,
}

//===============================================================================
/// Implementation of `triangle_fuzzy_number`.
//
impl<
        T: FuzzyNumberTrait
            + std::ops::Add<Output = T>
            + std::ops::Sub
            + std::ops::Mul
            + std::ops::Div<Output = T>,
    > TriangleFuzzyNumber<T>
where
    TriangleFuzzyNumber<T>: IntoIterator,
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
    pub fn ranking_function(self: TriangleFuzzyNumber<T>) -> f64
    where
        T: Into<f64>,
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
impl<T: Copy + FuzzyNumberTrait> Copy for TriangleFuzzyNumber<T> {}

//-------------------------------------------------------------------------------
/// Implementation of overloaded clone trait for `triangle_fuzzy_number`.
//
impl<T: Clone + FuzzyNumberTrait> Clone for TriangleFuzzyNumber<T> {
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
impl<T: ops::Add<Output = T> + FuzzyNumberTrait> ops::Add<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
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
impl<T: Copy + ops::Add<Output = T> + FuzzyNumberTrait> ops::AddAssign<TriangleFuzzyNumber<T>>
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
impl<T: ops::Sub<Output = T> + FuzzyNumberTrait> ops::Sub<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
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
impl<T: Copy + ops::Sub<Output = T> + FuzzyNumberTrait> ops::SubAssign<TriangleFuzzyNumber<T>>
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
impl<T: ops::Mul<Output = T> + FuzzyNumberTrait> ops::Mul<TriangleFuzzyNumber<T>>
    for TriangleFuzzyNumber<T>
{
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
impl<T: Copy + ops::Mul<Output = T> + FuzzyNumberTrait> ops::MulAssign<TriangleFuzzyNumber<T>>
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
impl<T: FuzzyNumberTrait> ops::Index<usize> for TriangleFuzzyNumber<T> {
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
impl<T: FuzzyNumberTrait> ops::IndexMut<usize> for TriangleFuzzyNumber<T> {
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
// Implementation of overloaded equality `triangle_fuzzy_number`.

//-------------------------------------------------------------------------------
/// Implementation of the inequality operator for `triangular_fuzzy_number`.
//
impl<
        T: FuzzyNumberTrait
            + std::cmp::PartialEq
            + Copy
            + std::ops::Sub
            + std::ops::Mul
            + std::ops::Add<Output = T>
            + std::ops::Div<Output = T>
            + Into<f64>,
    > PartialOrd for TriangleFuzzyNumber<T>
{
    fn partial_cmp(&self, other: &TriangleFuzzyNumber<T>) -> Option<std::cmp::Ordering> {
        let s = self.ranking_function();
        let o = other.ranking_function();
        return Some(s.total_cmp(&o));
    }
}

//===============================================================================
// Implementation of overloaded iterator for `triangle_fuzzy_number`.

//-------------------------------------------------------------------------------
/// Implementation of the into iterator operator for `triangular_fuzzy_number`.
//
impl<T: FuzzyNumberTrait + Copy> IntoIterator for TriangleFuzzyNumber<T> {
    type Item = T;
    type IntoIter = TriangleFuzzyNumberIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TriangleFuzzyNumberIntoIterator {
            tfn: self,
            index: 0,
        }
    }
}

//-------------------------------------------------------------------------------
/// Implementation of the iterator operator for `triangular_fuzzy_number`.
//
impl<T: FuzzyNumberTrait + Copy> Iterator for TriangleFuzzyNumberIntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = match self.index {
            0 => self.tfn.a,
            1 => self.tfn.b,
            2 => self.tfn.c,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
