#![feature(string_remove_matches)]

mod model;
mod regex_pattern;
mod test;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use model::ParsedLineResult;
use crate::regex_pattern::{REGEX_ACCEL_X, REGEX_ACCEL_Y, REGEX_ACCEL_Z, REGEX_APP_STRING, REGEX_BPM_HEART_RATE, REGEX_UNIX_EPOCH_MILLI};

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
            if let Some(line_result) = extract_from_line(&line.unwrap()) {}
        }
    )
}

fn extract_from_line(line: &str) -> Option<ParsedLineResult> {
    println!("{}", line);

    // Entry Point for Data Extraction from Log
    let re_app_str = Regex::new(REGEX_APP_STRING).unwrap();
    if let Some(_) = re_app_str.find(line) {
        let re_epoch_millies = Regex::new(REGEX_UNIX_EPOCH_MILLI).unwrap();
        if let Some(unix_nano) = re_epoch_millies.find(line) {
            let mut trimmed_epoch = String::from(unix_nano.as_str());
            trimmed_epoch.remove_matches("unix: ");
            println!("{}", trimmed_epoch);

            let mut line_result = ParsedLineResult::new(
                trimmed_epoch
                    .as_str()
                    .parse::<i64>()
                    .unwrap());

            let re_bpm = Regex::new(REGEX_BPM_HEART_RATE).unwrap();
            if let Some(bpm) = re_bpm.find(line) {
                let mut trimmed_bpm = String::from(bpm.as_str());
                trimmed_bpm.remove_matches("bpm: ");
                println!("{}", trimmed_bpm);
                line_result.bpm = Option::from(trimmed_bpm.as_str().parse::<u8>().unwrap());
            }


            // Check for Values of Acceleration
            let re = Regex::new(REGEX_ACCEL_X).unwrap();
            if let Some(accel_x) = re.find(line) {
                let mut trimmed = String::from(accel_x.as_str());
                trimmed.remove_matches("accel_x: ");
                println!("{}", trimmed);
                line_result.accel_x = Option::from(trimmed.as_str().parse::<f32>().unwrap());
            }

            let re = Regex::new(REGEX_ACCEL_Y).unwrap();
            if let Some(accel_y) = re.find(line) {
                let mut trimmed = String::from(accel_y.as_str());
                trimmed.remove_matches("accel_y: ");
                println!("{}", trimmed);
                line_result.accel_y = Option::from(trimmed.as_str().parse::<f32>().unwrap());
            }

            let re = Regex::new(REGEX_ACCEL_Z).unwrap();
            if let Some(accel_z) = re.find(line) {
                let mut trimmed = String::from(accel_z.as_str());
                trimmed.remove_matches("accel_z: ");
                println!("{}", trimmed);
                line_result.accel_z = Option::from(trimmed.as_str().parse::<f32>().unwrap());
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
