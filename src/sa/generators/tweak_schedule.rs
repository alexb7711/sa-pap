//===============================================================================
// Import standard library modules
// use crate::util::rand_utils;
use rand::distributions::{Distribution, Standard, WeightedIndex};
use rand::prelude::*;
use rand::Rng;
use strum::{EnumIter, IntoEnumIterator};

//===============================================================================
// Import developed modules
use crate::sa::charger::Charger;
use crate::sa::generators::primitives::new_charger::*;
use crate::sa::generators::primitives::new_window::*;
use crate::sa::generators::primitives::slide_visit::*;
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
pub struct TweakSchedule {}

//===============================================================================
/// Implementation of `TweakSchedule`
//
impl TweakSchedule {
    //---------------------------------------------------------------------------
    /// Initialize the `TweakSchedule` object
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `TweakSchedule`: Simulated annealing structure
    ///
    pub fn new() -> TweakSchedule {
        return TweakSchedule {};
    }
}

//===============================================================================
/// Implementation of `Generator` for `TweakSchedule`
//
impl Generator for TweakSchedule {
    fn run(self: &mut TweakSchedule, r: &mut Box<dyn Route>, c: &mut Charger) -> bool {
        // Get the data
        let mut rd = r.get_data();
        let N = rd.param.N;
        let k = rd.param.k[0];
        let nu = rd.param.nu;
        let gam = &rd.param.gam;
        let Gam = &rd.param.Gam;
        let eta = &rd.dec.eta;

        // Track the success of tweak
        let success: bool;

        // Create a vector of `Primitives` and shuffle the vector
        let primitives = Primitives::iter().collect::<Vec<_>>();
        let prim_weight = [3, 3, 2, 1];
        let prim_dist = WeightedIndex::new(&prim_weight).unwrap();
        let mut priority_id: Vec<usize> = vec![];

        let idx_weight: Vec<f32> = rd
            .dec
            .eta
            .clone()
            .iter()
            .enumerate()
            .map(|(idx, x)| {
                let idx = N - idx - 1;
                if *x <= nu * k
                    || (gam[idx] >= 0 && eta[gam[idx] as usize] <= nu * k)
                    || priority_id.contains(&(Gam[idx] as usize))
                {
                    if !priority_id.contains(&(Gam[idx] as usize)) {
                        priority_id.push(idx);
                    }

                    x.abs() * k
                } else if *x > nu * k {
                    x.abs() / k
                } else {
                    k
                }
            })
            .collect();
        let idx_dist = WeightedIndex::new(&idx_weight).unwrap();

        // Create random generate
        let mut rng = thread_rng();

        // Get random visit
        let ri = idx_dist.sample(&mut rng);
        let q = rd.dec.v[ri];
        let id = rd.param.Gam[ri] as usize;
        let ud = &(rd.dec.u[ri], rd.dec.d[ri]);
        let ae = &(rd.param.a[ri], rd.param.e[ri]);

        // Loop through the primitives
        let p = primitives[prim_dist.sample(&mut rng)].clone();

        // Try running the primitive and store the result
        success = match p {
            Primitives::NewCharger => new_charger::run(&mut rd, ri, c, q, id, ud),
            Primitives::NewWindow => new_window::run(&mut rd, ri, c, q, id, ae, ud),
            Primitives::Wait => wait::run(&mut rd, ri, c, q, id, ae, ud),
            Primitives::SlideVisit => slide_visit::run(&mut rd, ri, c, id, q, ae, ud),
        };

        // If successful, update the MILP data and break out of loop
        if success {
            r.set_data(rd.clone());
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
