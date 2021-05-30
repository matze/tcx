use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(long, short, parse(from_os_str))]
    input: PathBuf,
}

struct Pace {
    hours: u8,
    mins: u8,
    secs: u8,
}

impl Pace {
    fn from_duration(duration: &std::time::Duration) -> Self {
        let total_secs = duration.as_secs();
        let hours = total_secs / 60 / 60;
        let mins = (total_secs - hours * 60 * 60) / 60;
        let secs = total_secs - hours * 60 * 60 - mins * 60;

        Self {
            hours: hours as u8,
            mins: mins as u8,
            secs: secs as u8,
        }
    }
}

impl fmt::Display for Pace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours == 0 {
            write!(f, "{}:{} min/km", self.mins, self.secs)
        }
        else {
            write!(f, "{}:{}:{} min/km", self.hours, self.mins, self.secs)
        }
    }
}

fn main() -> Result<()> {
    let opt = Options::from_args();
    let reader = BufReader::new(File::open(opt.input)?);
    let db = tcx::Database::new(reader)?;

    for activity in db.activities {
        let activity = activity.activity;
        let duration = activity.duration();
        let seconds = duration.num_seconds() % 60;
        let minutes = (duration.num_seconds() / 60) % 60;
        let hours = (duration.num_seconds() / 60) / 60;

        println!("{:?} ({:?})", activity.sport, activity.id);
        println!("  Distance: {:.2} km", activity.distance() / 1000.0);
        println!("  Time: {:02}:{:02}:{:02}", hours, minutes, seconds);
        println!("  Tempo: {}", Pace::from_duration(&activity.average_tempo()));
        println!("  Heart rate: {} bpm", activity.heart_rate());
        println!("  Energy: {} kcal", activity.calories());
        println!("  Cadence: {} steps/min", activity.cadence());
        println!(
            "  Incline: ⬈ {:.1}, ⬊ {:.1} m",
            activity.ascent(),
            activity.descent()
        );
    }

    Ok(())
}
