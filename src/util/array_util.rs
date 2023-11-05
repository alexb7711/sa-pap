//==============================================================================
/// The `array_util` module implements functionality that vectors and arrays
/// can use.
pub mod arry_util {

    //--------------------------------------------------------------------------
    /// The `first` function returns the first instance of an element in a
    /// vector.
    ///
    /// # Input
    /// * vec: Vector to search through
    /// * element: The element to search for
    ///
    /// # Output
    /// * index: The index of the first instance of `element`
    ///
    pub fn first<I>(vec: &Vec<I>, element: I) -> Option<usize>
    where
        I: PartialEq + Copy,
    {
        return vec.iter().position(|&i| i == element);
    }

    //--------------------------------------------------------------------------
    /// The `last` function returns the last instance of an element in a
    /// vector.
    ///
    /// # Input
    /// * vec: Vector to search through
    /// * element: The element to search for
    ///
    /// # Output
    /// * index: The index of the last instance of `element`
    ///
    pub fn last<I>(vec: &Vec<I>, element: I) -> Option<usize>
    where
        I: PartialEq + Copy,
    {
        // Copy the vector and reverse
        let vec = &mut vec.clone();
        vec.reverse();

        // Search for the first instance of the element in reverse order
        // If a match is made
        if let Some(i) = first(vec, element) {
            // Return the vector length minus the max index
            return Some(vec.len() - i - 1);
        // Otherwise the element was not found
        } else {
            // Return nothing
            return None;
        }
    }
}
