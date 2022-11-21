use chrono::prelude::*;

/// LogLine for BPM Data Content
struct BPMLogLine {
    time: DateTime<Utc>,
    bpm: usize,
}

/// LogLine for Acceleration Data Content
struct AccelLogLine {
    time: DateTime<Utc>,
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
}

/// LogLine for Gyroscopic Data Content
struct GyroLogLine {
    time: DateTime<Utc>,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
}


// temporary struct: hold the result of the regex findings
// easier classification of line content
#[derive(Debug)]
pub struct ParsedLineResult {
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
