//===============================================================================
// Import standard library modules
use crate::util::rand_utils;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use strum::{EnumIter, IntoEnumIterator};

//===============================================================================
// Import developed modules
use crate::sa::charger::Charger;
use crate::sa::generators::primitives::new_charger_quick::*;
use crate::sa::generators::primitives::new_visit_quick::*;
use crate::sa::generators::primitives::slide_visit_quick::*;
use crate::sa::generators::primitives::wait::*;
use crate::sa::generators::Generator;
use crate::sa::route::Route;

//===============================================================================
/// Structure defining the information to create a charge schedule
//
#[derive(Clone, Debug, EnumIter)]
enum Primitives {
    NewCharger,
    NewWindow,
    Wait,
    SlideVisit,
}

//===============================================================================
/// Implementation of `Distribution` for `Primitives` enum
//
impl Distribution<Primitives> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Primitives {
        match rng.gen_range(0..3) {
            0 => Primitives::NewCharger,
            1 => Primitives::NewWindow,
            2 => Primitives::Wait,
            _ => Primitives::SlideVisit,
        }
    }
}

//===============================================================================
/// Structure defining the information to create a charge schedule
//
#[derive(Default)]
pub struct TweakScheduleQuick {}

//===============================================================================
/// Implementation of `TweakScheduleQuick`
//
impl TweakScheduleQuick {
    //---------------------------------------------------------------------------
    /// Initialize the `TweakScheduleQuick` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `TweakScheduleQuick`: Simulated annealing structure
    ///
    pub fn new() -> TweakScheduleQuick {
        return TweakScheduleQuick {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `TweakScheduleQuick`
//
impl Generator for TweakScheduleQuick {
    fn run(self: &mut TweakScheduleQuick, r: &mut Box<dyn Route>, c: &mut Charger) -> bool {
        // Track the success of tweak
        let mut success: bool = false;

        // Create a vector of `Primitives` and shuffle the vector
        let primitives = Primitives::iter().collect::<Vec<_>>();
        let primitives = rand_utils::shuffle_vec(&primitives);

        // Get random visit
        let mut rd = r.get_data();
        let ri = rand::thread_rng().gen_range(0..rd.param.N);
        let q = rd.dec.v[ri];
        let id = rd.param.Gam[ri] as usize;
        let ud = &(rd.dec.u[ri], rd.dec.d[ri]);
        let ae = &(rd.param.a[ri], rd.param.e[ri]);

        // Loop through the primitives
        for p in primitives {
            // Try running the primitive and store the result
            success = match p {
                Primitives::NewCharger => new_charger_quick::run(&mut rd, ri, c, q, id, ud),
                Primitives::NewWindow => new_visit_quick::run(&mut rd, ri, c, q, id, ae, ud),
                Primitives::Wait => wait::run(&mut rd, ri, c, q, id, ae, ud),
                Primitives::SlideVisit => slide_visit_quick::run(&mut rd, ri, c, id, q, ae, ud),
            };

            // If successful, update the MILP data and break out of loop
            if success {
                r.set_data(rd.clone());
                break;
            }
        }

        return success;
    }
}

//===============================================================================
// TESTS
#[cfg(test)]
mod priv_test_route_gen {
    use super::Primitives;

    #[test]
    fn test_primitive_sample() {
        for _ in 0..100 {
            // Create a Primitive enumeration
            let p: Primitives = rand::random();
            let p: usize = p as usize;

            // Test 0 - Make sure the sample is within range
            assert!(p < 4, "{} is not less than 4.", p);
        }
    }
}
