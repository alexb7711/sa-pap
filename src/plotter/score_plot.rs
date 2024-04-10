#![allow(non_snake_case)]

//===============================================================================
// Standard library
use gnuplot::*;

//===============================================================================
// Import modules
use crate::plotter::Plotter;
use crate::sa::data::Data;

//===============================================================================
/// Structure for `accumulated_energy_usage_plot`
pub struct ScorePlot {}

//===============================================================================
//
impl ScorePlot {
    //--------------------------------------------------------------------------
    /// Implementation of the plotting function to display the schedule of the BEBs.
    /// The plot consists of
    ///
    /// # Input
    /// * d: Boxed data
    ///
    /// # Output
    /// * Schedule plot
    ///
    ///
    fn create_plot(dat: &mut Box<Data>, fg: &mut Figure) {
        // Variables
        let bscore: &Vec<f64> = &dat.dec.Jb;
        let cscore: &Vec<f64> = &dat.dec.Jc;
        let nscore: &Vec<f64> = &dat.dec.Jn;

        // Create domain
        let x: Vec<f32> = (0..cscore.len()).map(|x| x as f32).collect();
        let zero: Vec<f32> = vec![0.0; cscore.len()];

        // Configure the plot
        let name: String = String::from("Score");
        fg.axes2d()
            .set_title(name.as_str(), &[])
            .set_x_label("Iteration", &[])
            .set_y_label("Score", &[])
            .set_legend(
                gnuplot::Graph(0.9),
                gnuplot::Graph(1.0),
                &[Placement(AlignLeft, AlignBottom)],
                &[TextAlign(AlignRight)],
            )
            .fill_between(
                x.clone(),
                bscore.clone(),
                nscore,
                &[
                    Caption("Candidate"),
                    Color("green"),
                    FillRegion(Below),
                    FillAlpha(0.15),
                ],
            )
            .lines(x.clone(), nscore, &[Color("green"), LineWidth(3.0)])
            .fill_between(
                x.clone(),
                bscore.clone(),
                cscore,
                &[
                    Color("blue"),
                    FillRegion(Below),
                    FillAlpha(0.50),
                    Caption("Active"),
                ],
            )
            .fill_between(
                x.clone(),
                zero.clone(),
                bscore,
                &[
                    Color("red"),
                    FillRegion(Below),
                    FillAlpha(0.25),
                    Caption("Best"),
                ],
            );
    }

    //--------------------------------------------------------------------------
    /// The `save_do_disk` function outputs the results of the plot to disk.
    ///
    /// # Input
    /// * fg: Figure for charger schedule
    ///
    /// # Output
    /// * NONE
    ///
    fn save_to_disk(fg: &Figure, p: &String) {
        // Save GNUPlot
        let name: String = String::from("slow-charger-power-usage");
        fg.echo_to_file(&format!("{}.gnuplot", p.clone() + name.as_str()));
    }
}

//===============================================================================
/// Implementation of the plotting function for the total accumulated energy.
/// That it, this line graph tracks the total energy consumed by the system over
/// the working day.
///
/// # Input
/// * d: Boxed data
///
/// # Output
/// * Power usage plot
///
impl Plotter for ScorePlot {
    fn plot(display_plot: bool, dat: &mut Box<Data>, p: &String) {
        // Configure plot
        let mut fg = Figure::new();

        // Create plot
        ScorePlot::create_plot(dat, &mut fg);

        // Plot Figure
        if display_plot {
            fg.show().unwrap();
        }

        // Save to disk
        ScorePlot::save_to_disk(&fg, p);
    }

    //===============================================================================
    //
    fn real_time(rpt: bool, dat: &mut Box<Data>, fg: &mut Figure) {
        // Determine whether to create the plot
        if !rpt {
            return;
        }

        // Clear plots
        fg.clear_axes();

        // Create plot
        ScorePlot::create_plot(dat, fg);

        // Update plots
        fg.show_and_keep_running().unwrap();
    }
}
