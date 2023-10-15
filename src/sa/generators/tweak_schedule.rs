//===============================================================================
// Import modules
use crate::sa::charger::Charger;
use crate::sa::route::Route;
use crate::sa::generators::Generator;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::sa::generators::primitives::new_charger::*;
use crate::sa::generators::primitives::new_visit::*;
use crate::sa::generators::primitives::new_window::*;
use crate::sa::generators::primitives::remove::*;
use crate::sa::generators::primitives::slide_visit::*;

//===============================================================================
/// Structure defining the information to create a charge schedule
//
enum Primitives {
    NewCharger, NewWindow, SlideVisit
}


//===============================================================================
/// Implementation of `Distribution` for `Primitives` enum
//
impl Distribution<Primitives> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Primitives {
        match rng.gen_range(0..2) {
            0 => Primitives::NewCharger,
            1 => Primitives::NewWindow,
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
    fn run(self: &mut TweakSchedule, _s: &mut dyn Route, c: &mut Charger) -> bool {
        let p: Primitives = rand::random();

        match p {
            Primitives::NewCharger => {
                new_charger::run(c, 0, 0, &(0.0, 0.0))
            },
            Primitives::NewWindow  => {
                new_window::run(c, 0, &(0.0, 0.0), &(0.0, 0.0))
            },
            Primitives::SlideVisit => {
                slide_visit::run(c, 0, 0, &(0.0, 0.0), &(0.0, 0.0))
            },
        }
    }
}
