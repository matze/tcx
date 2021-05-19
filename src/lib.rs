use chrono::prelude::*;
use serde::Deserialize;
use std::io::Read;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("XML Parse error: {0}")]
    Parse(#[from] serde_xml_rs::Error)
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

impl Database {
    pub fn new<R: Read>(reader: R) -> Result<Self, Error> {
        Ok(serde_xml_rs::from_reader(reader)?)
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
}