extern crate sa_pap;

//===============================================================================
//
#[cfg(test)]
mod test_charger {
    //---------------------------------------------------------------------------
    // Import modules
    use super::sa_pap::sa::charger::Assignment;
    use super::sa_pap::sa::charger::Charger;

    //---------------------------------------------------------------------------
    //
    #[test]
    fn test_csv_load() {
        // Create charger
        let mut charger: Charger = Charger::new(None);

        // Make sure we have an empty charger queue
        assert_eq!(charger.schedule.is_empty(), false);

        // Test assign charger
        let q: usize = 0;
        let c: (f32, f32) = (0.1, 0.2);
        let id: usize = 3;

        // Ensure that the charger space is available
        assert!(charger.avail(&q, &c));

        // Assign the charger
        charger.assign(q, c, id);

        assert_eq!(charger.schedule[0][0], Assignment { t: c, b: id });
    }
}
