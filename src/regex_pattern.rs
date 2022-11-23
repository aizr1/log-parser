// LOG LINE ATTRIBUTE REGEX
pub const REGEX_TIME: &str = r"\[(\d{2}):(\d{2}):(\d{2})\]";
pub const REGEX_LOG_LEVEL: &str = r"\[(\binfo\b|\bwarn\b)\]";
pub const REGEX_APP_STRING: &str = r"App:";

// DIRTY LOOKUP OF UNIX EPOCH IN MILLISECONDS
// unix: 1667481423234
pub const REGEX_UNIX_EPOCH_MILLI: &str = r"(unix:)( )(\d{13})";

// BPM DATA REGEX
// bpm: [range 30-250]
pub const REGEX_BPM_HEART_RATE: &str = r"(bpm:)( )(\d{2,3})";

// GYRO DATA REGEX
pub const REGEX_GYRO_X: &str = r"(gyro_x:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_GYRO_Y: &str = r"(gyro_y:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_GYRO_Z: &str = r"(gyro_z:)( )([+-]?([0-9]*[.])?[0-9]+)";

// ACCELERATION DATA REGEX
// Regex seaches for indicator word in first group, then tries to find the floating point number
// string in the second capturing group
pub const REGEX_ACCEL_X: &str = r"(accel_x:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_ACCEL_Y: &str = r"(accel_y:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_ACCEL_Z: &str = r"(accel_z:)( )([+-]?([0-9]*[.])?[0-9]+)";
