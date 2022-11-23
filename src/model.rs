use chrono::prelude::*;
use serde::Serialize;

/// LogLine for BPM Data Content
#[derive(Debug, Serialize)]
pub struct BPMLogLine {
    pub epoch: i64,
    pub time: DateTime<Utc>,
    pub bpm: u8,
}

/// LogLine for Acceleration Data Content
#[derive(Debug, Serialize)]
pub struct AccelLogLine {
    pub epoch: i64,
    pub time: DateTime<Utc>,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
}

/// LogLine for Gyroscopic Data Content
#[derive(Debug, Serialize)]
pub struct GyroLogLine {
    pub epoch: i64,
    pub time: DateTime<Utc>,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}


// temporary struct: hold the result of the regex findings
// easier classification of line content
#[derive(Debug, Serialize)]
pub struct ParsedLineResult {
    pub epoch: i64,
    pub time: DateTime<Utc>,
    pub bpm: Option<u8>,
    pub accel_x: Option<f32>,
    pub accel_y: Option<f32>,
    pub accel_z: Option<f32>,
    pub gyro_x: Option<f32>,
    pub gyro_y: Option<f32>,
    pub gyro_z: Option<f32>,
}

impl ParsedLineResult {
    pub fn new(epoch_millis: i64) -> Self {
        let time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_millis(
                epoch_millis,
            )
                .unwrap(),
            Utc);
        Self {
            epoch: epoch_millis,
            time,
            bpm: None,
            accel_x: None,
            accel_y: None,
            accel_z: None,
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
        }
    }
}

pub enum OutputDataType {
    BPM(BPMLogLine),
    ACCEL(AccelLogLine),
    GYRO(GyroLogLine),
    IRRELEVANT
}
