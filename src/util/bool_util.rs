//===============================================================================
/// Returns false if int == 0, true otherwise
///
/// # Input
/// * int: i64 integer
///
/// # Output
/// * is_true
///
pub fn i64_to_bool(int: i64) -> bool
{
    // Local variables
    let mut is_true: bool = true;

    if int == 0
    {
        is_true = false
    }

    return is_true;
}
