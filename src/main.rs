#![feature(string_remove_matches)]

mod log_classification;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use regex::Regex;

pub const REGEX_TIME: &str = r"\[(\d{2}):(\d{2}):(\d{2})\]";
pub const REGEX_LOG_LEVEL: &str = r"\[(\binfo\b|\bwarn\b)\]";
pub const REGEX_APP_STRING: &str = r"App:";
// unix: 1667481423234
pub const REGEX_UNIX_EPOCH_MILLI: &str = r"(unix:)( )(\d{13})";

// bpm: [range 30-250]
pub const REGEX_BPM_HEART_RATE: &str = r"(bpm:)( )(\d{2,3})";

// GYRO
pub const REGEX_GYRO_X: &str = r"(gyro_x:)( )(\d{10,12})";
pub const REGEX_GYRO_Y: &str = r"(gyro_y:)( )(\d{10,12})";
pub const REGEX_GYRO_Z: &str = r"(gyro_z:)( )(\d{10,12})";

// ACCEL
// Regex seaches for indicator word in first group, then tries to find the floating point number string
// in the second capturing group
pub const REGEX_ACCEL_X: &str = r"(accel_x:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_ACCEL_Y: &str = r"(accel_y:)( )([+-]?([0-9]*[.])?[0-9]+)";
pub const REGEX_ACCEL_Z: &str = r"(accel_z:)( )([+-]?([0-9]*[.])?[0-9]+)";


fn main() {
    let filename = "Test.log";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    // Read the File to a buffered reader for efficiency
    let reader = BufReader::new(file);

    // Iterate the lines of the buffered file
    reader.lines().for_each(
        |line| {
            // Parse every line using a regex with capture groups
            extract_from_line(&line.unwrap());
        }
    )
}


// temporary struct: hold the result of the regex findings
// easier classification of line content
#[derive(Debug)]
struct LineResult {
    time: DateTime<Utc>,
    bpm: Option<u8>,
    accel_x: Option<f32>,
    accel_y: Option<f32>,
    accel_z: Option<f32>,
    gyro_x: Option<f32>,
    gyro_y: Option<f32>,
    gyro_z: Option<f32>,
}

impl LineResult {
    fn new(epoch_millis: i64) -> Self {
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

fn extract_from_line(line: &str) -> Option<LineResult> {
    println!("{}", line);

    // Entry Point for Data Extraction from Log
    let re_app_str = Regex::new(REGEX_APP_STRING).unwrap();
    if let Some(_) = re_app_str.find(line.clone()) {
        let re_epoch_millies = Regex::new(REGEX_UNIX_EPOCH_MILLI).unwrap();
        if let Some(unix_nano) = re_epoch_millies.find(line.clone()) {
            let mut trimmed_epoch = String::from(unix_nano.as_str());
            trimmed_epoch.remove_matches("unix: ");
            println!("{}", trimmed_epoch);

            let mut line_result = LineResult::new(
                trimmed_epoch
                    .as_str()
                    .clone()
                    .parse::<i64>()
                    .unwrap());

            let re_bpm = Regex::new(REGEX_BPM_HEART_RATE).unwrap();
            if let Some(bpm) = re_bpm.find(line.clone()) {
                let mut trimmed_bpm = String::from(bpm.as_str());
                trimmed_bpm.remove_matches("bpm: ");
                println!("{}", trimmed_bpm);
                line_result.bpm = Option::from(trimmed_bpm.as_str().clone().parse::<u8>().unwrap());
            }


            // Check for Values of Acceleration
            let re = Regex::new(REGEX_ACCEL_X).unwrap();
            if let Some(accel_x) = re.find(line.clone()) {
                let mut trimmed = String::from(accel_x.as_str());
                trimmed.remove_matches("accel_x: ");
                println!("{}", trimmed);
                line_result.accel_x = Option::from(trimmed.as_str().clone().parse::<f32>().unwrap());
            }

            let re = Regex::new(REGEX_ACCEL_Y).unwrap();
            if let Some(accel_y) = re.find(line.clone()) {
                let mut trimmed = String::from(accel_y.as_str());
                trimmed.remove_matches("accel_y: ");
                println!("{}", trimmed);
                line_result.accel_y = Option::from(trimmed.as_str().clone().parse::<f32>().unwrap());
            }

            let re = Regex::new(REGEX_ACCEL_Z).unwrap();
            if let Some(accel_z) = re.find(line.clone()) {
                let mut trimmed = String::from(accel_z.as_str());
                trimmed.remove_matches("accel_z: ");
                println!("{}", trimmed);
                line_result.accel_z = Option::from(trimmed.as_str().clone().parse::<f32>().unwrap());
            }

            println!("{:#?}", line_result);

            Some(line_result)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use regex::Regex;
    use crate::{capture_group_for_line, extract_from_line, REGEX_TIME};

    pub const TEST_LINE_0: &str = "[14:32:16][info][app]";
    pub const TEST_LINE_1: &str = "[14:32:16][info][app]    App:";
    pub const TEST_LINE_2: &str = "[14:32:16][info][app]    App:";
    pub const TEST_LINE_3: &str = "[14:32:16][info][app]    App: unix: 1667424212342 bpm: 85 lijlkj 0983098402983j lkejwrlkj";

    #[test]
    fn extract_time_test() {
        let res = extract_from_line(TEST_LINE_0);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare)
    }


    #[test]
    fn extract_app_line_test() {
        let res = extract_from_line(TEST_LINE_1);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare);

        let compare = "App:";
        assert_eq!(res[2], compare);
    }

    #[test]
    fn extract_app_line_test_2() {
        let res = extract_from_line(TEST_LINE_2);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare);

        let compare = "App:";
        assert_eq!(res[2], compare);
    }

    #[test]
    fn extract_app_line_test_3() {
        let res = extract_from_line(TEST_LINE_2);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare);

        let compare = "App:";
        assert_eq!(res[2], compare);
    }

    #[test]
    fn extract_app_line_test_4() {
        let res = extract_from_line(TEST_LINE_3);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare);

        let compare = "App:";
        assert_eq!(res[2], compare);

        let compare = "unix: 1667424212342";
        assert_eq!(res[3], compare)
    }

    #[test]
    fn extract_app_line_test_5() {
        let res = extract_from_line(TEST_LINE_3);
        let compare = "[14:32:16]";
        assert_eq!(res[0], compare);

        let compare = "[info]";
        assert_eq!(res[1], compare);

        let compare = "App:";
        assert_eq!(res[2], compare);

        let compare = "unix: 1667424212342";
        assert_eq!(res[3], compare);

        let compare = "bpm: 85";
        assert_eq!(res[4], compare);
    }
}
