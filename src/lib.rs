use chrono::prelude::*;
use serde::Deserialize;
use std::convert::From;
use std::io::Read;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("XML Parse error: {0}")]
    Parse(#[from] serde_xml_rs::Error),
}

#[derive(Deserialize, Debug)]
pub struct HeartRate {
    #[serde(rename = "Value")]
    pub value: i32,
}

#[derive(Deserialize, Debug)]
pub struct Position {
    #[serde(rename = "LatitudeDegrees")]
    pub lat: f64,

    #[serde(rename = "LongitudeDegrees")]
    pub lon: f64,
}

#[derive(Deserialize, Debug)]
pub enum SensorState {
    Present,
    Absent,
}

#[derive(Deserialize, Debug)]
pub struct Sample {
    #[serde(rename = "Time")]
    pub time: chrono::DateTime<Utc>,

    #[serde(rename = "Position")]
    pub position: Option<Position>,

    #[serde(rename = "AltitudeMeters")]
    pub altitude: Option<f64>,

    #[serde(rename = "HeartRateBpm")]
    pub heart_rate: HeartRate,

    #[serde(rename = "SensorState")]
    pub sensor_state: SensorState,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    #[serde(rename = "Trackpoint")]
    pub samples: Vec<Sample>,
}

#[derive(Deserialize, Debug)]
pub struct Lap {
    #[serde(rename = "TotalTimeSeconds")]
    pub time: f64,

    #[serde(rename = "DistanceMeters")]
    pub distance: f64,

    #[serde(rename = "Track")]
    pub track: Track,

    #[serde(rename = "Calories")]
    pub calories: i32,

    #[serde(rename = "Cadence")]
    pub cadence: i32,

    #[serde(rename = "AverageHeartRateBpm")]
    pub average_heart_rate: HeartRate,

    #[serde(rename = "MaximumHeartRateBpm")]
    pub maximum_heart_rate: HeartRate,
}

#[derive(Deserialize, Debug)]
pub enum Sport {
    Running,
    Biking,
    Other,
}

#[derive(Deserialize, Debug)]
pub struct Activity {
    #[serde(rename = "Sport")]
    pub sport: Sport,

    #[serde(rename = "Id")]
    pub id: chrono::DateTime<Utc>,

    #[serde(rename = "Lap")]
    pub laps: Vec<Lap>,
}

#[derive(Deserialize, Debug)]
pub struct Activities {
    #[serde(rename = "Activity")]
    pub activity: Activity,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "TrainingCenterDatabase")]
pub struct Database {
    #[serde(rename = "Activities")]
    pub activities: Vec<Activities>,
}

impl From<HeartRate> for i32 {
    fn from(heart_rate: HeartRate) -> Self {
        heart_rate.value
    }
}

impl Database {
    pub fn new<R: Read>(reader: R) -> Result<Self, Error> {
        Ok(serde_xml_rs::from_reader(reader)?)
    }
}

fn altitude_difference(a: &Sample, b: &Sample) -> Option<f64> {
    if let (Some(a), Some(b)) = (a.altitude, b.altitude) {
        return Some(a - b);
    }

    None
}

impl Track {
    /// Total ascent in meters.
    pub fn ascent(&self) -> f64 {
        self.samples
            .windows(2)
            .filter_map(|s| altitude_difference(&s[0], &s[1]))
            .filter(|d| d > &0.0)
            .sum()
    }

    /// Total descent in meters.
    pub fn descent(&self) -> f64 {
        self.samples
            .windows(2)
            .filter_map(|s| altitude_difference(&s[1], &s[0]))
            .filter(|d| d > &0.0)
            .sum()
    }
}

impl Activity {
    pub fn distance(&self) -> f64 {
        self.laps.iter().map(|l| l.distance).sum()
    }

    pub fn duration(&self) -> chrono::Duration {
        let secs = self.laps.iter().map(|l| l.time as u64).sum();
        chrono::Duration::from_std(Duration::from_secs(secs)).unwrap()
    }

    pub fn heart_rate(&self) -> i32 {
        if self.laps.is_empty() {
            return 0;
        }

        self.laps
            .iter()
            .map(|l| l.average_heart_rate.value)
            .sum::<i32>()
            / self.laps.len() as i32
    }

    pub fn calories(&self) -> i32 {
        self.laps.iter().map(|l| l.calories).sum()
    }

    pub fn cadence(&self) -> i32 {
        self.laps.iter().map(|l| l.cadence).sum::<i32>() / self.laps.len() as i32
    }

    /// Total ascent in meters.
    pub fn ascent(&self) -> f64 {
        self.laps.iter().map(|l| l.track.ascent()).sum()
    }

    /// Total descent in meters.
    pub fn descent(&self) -> f64 {
        self.laps.iter().map(|l| l.track.descent()).sum()
    }

    /// Average tempo in minutes per km.
    pub fn average_tempo(&self) -> Duration {
        let secs = self.laps
            .iter()
            .map(|l| l.time / (l.distance / 1000.0))
            .sum::<f64>()
            / self.laps.len() as f64;

        Duration::from_secs_f64(secs)
    }
}
