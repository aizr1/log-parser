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