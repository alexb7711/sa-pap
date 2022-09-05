extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_temp_func
{
    //---------------------------------------------------------------------------
    // Import modules
    use sa_pap::sa::temp_func::{TempFunc, CoolSchedule::*};

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_linear()
    {
        // Auto run
        let l: TempFunc = TempFunc::new(Linear, 500.0, 0.5, true);
        assert_eq!(l.get_temp_vec().unwrap().len(), 998);

        // Check temperature
        assert_eq!(l.get_temp(Some(0)).unwrap(), 500.0);
        assert_eq!(l.get_temp(None).unwrap(), 1.5);

        // Stepping
        let mut l: TempFunc = TempFunc::new(Linear, 500.0, 0.5, false);
        for _ in 1..10 { l.step(); }
        assert_eq!(l.get_temp_vec().unwrap().len(), 10);

    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_geometric()
    {
        // Auto run
        let g: TempFunc = TempFunc::new(Geometric, 500.0, 0.995, true);
        assert_eq!(g.get_temp_vec().unwrap().len(), 1240);

        // Check temperature
        assert_eq!(g.get_temp(Some(0)).unwrap(), 500.0);
        assert_eq!(g.get_temp(None).unwrap(), 1.0040821);


        // Stepping
        let mut g: TempFunc = TempFunc::new(Geometric, 500.0, 0.995, false);
        for _ in 1..10 { g.step(); }
        assert_eq!(g.get_temp_vec().unwrap().len(), 10);
    }

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_exponintial()
    {
        // Auto run
        let e: TempFunc = TempFunc::new(Exponential, 500.0, 0.01, true);
        assert_eq!(e.get_temp_vec().unwrap().len(), 622);

        // Check temperature
        assert_eq!(e.get_temp(Some(0)).unwrap(), 500.0);
        assert_eq!(e.get_temp(None).unwrap(), 1.0046227);

        // Stepping
        let mut e: TempFunc = TempFunc::new(Exponential, 500.0, 0.01, false);
        for _ in 1..10 { e.step(); }
        assert_eq!(e.get_temp_vec().unwrap().len(), 10);
    }
}
