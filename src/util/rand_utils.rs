// Import standard library
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

//===============================================================================
/// Creates a list of `vec_size` values that sum up `sum_val`
///
/// # Input
/// * `vec_size` : Size of vec
/// * `sum_val`  : Value that all numbers in vec add up to
///
/// # Output
/// * `rand_vec`: vec of random values that add up to `sum_val`
///
pub fn rand_route_count(vec_size: u16, sum_val: u16) -> Vec<u16> {
    // Variables
    let mut rand = thread_rng();
    let mut rand_vec: Vec<u16> = vec![1; vec_size as usize];
    let mut rand_id: u16;

    // Randomly apply a visits to each bus
    for _ in 0..sum_val - vec_size {
        rand_id = rand.gen_range(0..vec_size);
        rand_vec[rand_id as usize] += 1;
    }

    return rand_vec;
}

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
pub fn rand_range(lower_bound: f32, upper_bound: f32) -> f32 {
    return rand::thread_rng().gen_range(lower_bound..=upper_bound);
}

//===============================================================================
/// Returns a vector that has been randomized.
///
/// # Input
/// * `vec` : Vector
///
/// # Output
/// * `rand_vec`: randomized `vec`
///
/// # References:
/// https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it
///
pub fn shuffle_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    // Copy the vector
    let mut vec = (*vec).clone();

    // Shuffle the vector
    vec.shuffle(&mut thread_rng());

    return vec;
}
