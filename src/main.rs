#![feature(string_remove_matches)]

mod model;
mod regex_pattern;
mod test;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use chrono::{Datelike, DateTime, NaiveDateTime, Timelike, TimeZone, Utc};
use chrono::format::parse;
use csv::Writer;
use regex::Regex;
use model::ParsedLineResult;
use crate::model::{AccelLogLine, BPMLogLine, OutputDataType};
use crate::model::OutputDataType::{ACCEL, BPM, GYRO, IRRELEVANT};
use crate::regex_pattern::{REGEX_ACCEL_X,
                           REGEX_ACCEL_Y,
                           REGEX_ACCEL_Z,
                           REGEX_APP_STRING,
                           REGEX_BPM_HEART_RATE,
                           REGEX_UNIX_EPOCH_MILLI};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to experiment
    #[arg(short, long, default_value="NameProbant")]
    name: String,

    /// Name of the log file to parse and export as csv
    #[arg(short, long, default_value="Test.log")]
    log_file: String,
}

fn main() {
    let args = Args::parse();

    let filename = args.log_file;
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    // Read the File to a buffered reader for efficiency
    let reader = BufReader::new(file);

    // Prepare file export
    let date: DateTime<Utc> = Utc::now();
    let now_str = format!("{}_{}", args.name, date.format("%Y-%m-%d_%H-%M-%S"));
    let mut bpm_writer = csv::Writer::from_path(format!("bpm-{}.csv", now_str))
        .expect("cannot init bpm csv writer");
    let mut accel_writer = csv::Writer::from_path(format!("accel-{}.csv", now_str))
        .expect("cannot init accel csv writer");

    // Iterate the lines of the buffered file
    for line in reader.lines() {
        // Parse every line using a regex with capture groups
        if let Some(line_result) = extract_from_line(&line.unwrap()) {
            match parse_line_result(line_result) {
                BPM(bpm_log_line) => export_bpm_csv(bpm_log_line, &mut bpm_writer),
                ACCEL(accel_log_line) => export_accel_csv(accel_log_line, &mut accel_writer),
                _ => {}
            }
        }
    }
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

fn parse_line_result(line_result: ParsedLineResult) -> OutputDataType {
    match line_result {

        // We are interested in BPM and Time
        ParsedLineResult {
            time,
            bpm: Some(bpm),
            accel_x: None,
            accel_y: None,
            accel_z: None,
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
        } => {
            BPM(BPMLogLine { time, bpm })
        }

        ParsedLineResult {
            time,
            bpm: None,
            accel_x: Some(accel_x),
            accel_y: Some(accel_y),
            accel_z: Some(accel_z),
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
        } => {
            ACCEL(AccelLogLine {
                time,
                accel_x,
                accel_y,
                accel_z,
            })
        }

        // Ignore all other struct shapes
        _ => {
            IRRELEVANT
        }
    }
}

fn export_bpm_csv(bpm_log_line: BPMLogLine, writer: &mut Writer<File>) {
    writer.serialize(bpm_log_line).expect("cannot write bpm csv line");
}

fn export_accel_csv(accel_log_line: AccelLogLine, writer: &mut Writer<File>) {
    writer.serialize(accel_log_line).expect("cannot write accel csv line");
}
