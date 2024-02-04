//===============================================================================
// Import developed modules
use crate::lp::constraints::Constraint;
use crate::sa::charger::Charger;
use crate::sa::data::Data;

//===============================================================================
/// Structure defining the information to calculate service time
//
pub struct ScalarToVectorQueue {}

//===============================================================================
/// Implementation of `Constraint` for `ScalarToVectorQueue` structure.
///
/// # Input
/// * d: Data for the current model
/// * i: index of the visit
/// * j: index for the queue
///
/// # Output
/// * bool: Constraint successfully applied and is true
///
#[allow(non_snake_case)]
impl Constraint for ScalarToVectorQueue {
    fn run(d: &mut Data, _ch: &mut Charger, i: usize, _: usize) -> bool {
        // Extract decision variables
        let v = &d.dec.v;
        let w = &mut d.dec.w;

        // Constraint

        // Determine the queue vector `w` for visit `i`
        if w[i][v[i]] != true {
            println!("Visit {}", i);
            println!("w[i][v[i]] = {}", w[i][v[i]]);
            println!("v[i]       = {}", v[i]);
            println!("scalar_to_vector_queue.rs: w[i][v[i]] != v[i].");
            return false;
        }

        // Ensure the visit vector does not have simultaneous assignments
        // https://stackoverflow.com/questions/69847288/is-there-an-easy-way-to-count-booleans-in-rust/69847395?noredirect=1#comment123467398_69847395
        if w[i].clone().into_iter().filter(|a| *a).count() > 1 {
            println!("Visit {}", i);
            println!("Charger indices that are true:");
            for (idx, itr) in w[i].iter().enumerate() {
                if *itr == false {
                    println!("- {}", idx);
                }
            }
            println!("scalar_to_vector_queue.rs: sum(w[i]) > 1");
            return false;
        }

        return true;
    }
}
