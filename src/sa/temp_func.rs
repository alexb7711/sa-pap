//===============================================================================
/// Types of cooling schedules
pub enum CoolSchedule
{
    Linear,
    Geometric,
    Exponential
}

//===============================================================================
/// Values required by temperature functions
pub struct TempFunc
{
    autorun  : bool,                                                            // Decide to generate entire array
    schedule : CoolSchedule,                                                    // Schedule type
    c_step   : usize,                                                           // Current step
    d        : f32,                                                             // Step parameter
    t        : Vec<f32>,                                                        // Current temperature
}

//===============================================================================
/// Implementation of temperature function.
impl TempFunc
{
    // Constants
    const EXP_MAX : f32  = 0.9999;
    const MIN_TEMP: f32  = 1.0;

    //---------------------------------------------------------------------------
    /// Initialize the linear temperature function
    ///
    /// # Input
    /// * `schedule`  : `CoolSchedule` type
    /// * `init_temp` : Initial temperature
    /// * `delta`     : Step size
    /// * `autorun`   : Decide whether to auto generate array of temps (default no)
    ///
    /// # Output
    /// * NONE
    ///
    pub fn new(schedule: CoolSchedule, init_temp: f32, delta: f32, autorun: bool) -> TempFunc
    {
        // Check delta based on the `Coolschedule`
        let d: f32 = match schedule
        {
            CoolSchedule::Linear      => delta.abs(),                           // Linear should be positive
            CoolSchedule::Geometric   =>                                        // Geometric should be less than 1
            {
                if delta.abs() > TempFunc::EXP_MAX { TempFunc::EXP_MAX }
                else                               { delta.abs() }
            }
            CoolSchedule::Exponential => -delta.abs(),                          // Exponential should be negative
        };

        // Create `TempFunc`
        let mut tf: TempFunc = TempFunc
        {
            autorun                   ,
            schedule                  ,
            c_step  : 0               ,
            d                         ,
            t       : vec!(init_temp) ,
        };

        // Execute autorun if true
        if autorun { TempFunc::autorun(&mut tf); }

        return tf;
    }

    //---------------------------------------------------------------------------
    /// Increment the temperature schedule.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `temp` : Temperature of next step
    ///
    pub fn step(self: &mut TempFunc) -> Option<f32>
    {
        // If autorun has not been executed, allow steps
        if !self.autorun && *self.t.last().unwrap() > TempFunc::MIN_TEMP
        {
            // Update the temperature
            return self.exec_cool_schedule();
        }

        return None;
    }

    //---------------------------------------------------------------------------
    /// Get the temperature from any step.
    ///
    /// # Input
    /// * `step` : Optional argument to get temperature from a specific step,
    ///            the default value gets the current temperature.
    ///
    /// # Output
    /// * `temp` : Temperature from step `step`.
    ///
    pub fn get_temp(self: &TempFunc, step: Option<isize>) -> Result<f32, &str>
    {
        // Variables
        let s: isize;

        // Set the step to the appropriate value
        match step
        {
            Some(x) => s = x,
            None    => s = self.c_step as isize,
        }

        // Check if the step provided is valid
        if (s as usize) < self.t.len()
        {
            return Ok(self.t[s as usize]);
        }
        else
        {
            return Err("Invalid step");
        }
    }

    //---------------------------------------------------------------------------
    /// Get the entire temperature vector
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * `temp_vec` : Temperature vector
    ///
    pub fn get_temp_vec(self: &TempFunc) -> Option<Vec<f32>>
    {
        if self.t.is_empty() {return None;}
        else                 { return Some(self.t.clone()); }
    }

    //---------------------------------------------------------------------------
    /// Generate the entire array of temperatures.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn autorun(self: &mut TempFunc)
    {
        while self.exec_cool_schedule().is_some() {}
    }

    //---------------------------------------------------------------------------
    /// Execute the cooling schedule.
    ///
    /// # Input
    /// * NONE
    ///
    /// # Output
    /// * NONE
    ///
    fn exec_cool_schedule(self: &mut TempFunc) -> Option<f32>
    {
        // Execute cooling schedule
        let t: f32 = match self.schedule
        {
            CoolSchedule::Exponential => self.d.exp()*self.t[self.c_step],
            CoolSchedule::Geometric   => self.d*self.t[self.c_step],
            CoolSchedule::Linear      => self.t[self.c_step] - self.d,
        };

        // Check if the temperature is valid
        if t <= TempFunc::MIN_TEMP { return None; }

        // Increment the step
        self.c_step += 1;

        // Update temperature
        self.t.append(&mut vec![t]);

        // Return temp
        return Some(t);
    }
}
