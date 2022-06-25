use rand::{thread_rng, Rng};

//===============================================================================
/// Creates a list of `vec_size` values that sum up `sum_val`
///
/// # Input
/// * `vec_size`: Size of vec
/// * `sum_val`   : Value that all numbers in vec add up to
///
/// # Output
/// * `rand_vec`: vec of random values that add up to `sum_val`
///
pub fn rand_route_count(vec_size: i64, sum_val: i64) -> Vec<i64>
{
    // Variables
    let mut rand               = thread_rng();
    let mut rand_vec: Vec<i64> = vec![1; vec_size as usize];
    let mut rand_id   : i64;

    // Randomly apply a visits to each bus
    for _ in 0..sum_val-vec_size
    {
        rand_id                     = rand.gen_range(0..vec_size);
        rand_vec[rand_id as usize] += 1;
    }

    return rand_vec;
}
