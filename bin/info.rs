use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(long, short, parse(from_os_str))]
    input: PathBuf,
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
        println!("  Heart rate: {} bpm", activity.heart_rate());
    }

    Ok(())
}
