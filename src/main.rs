use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

pub const REGEX_TIME: &str = r"\[(\d{2}):(\d{2}):(\d{2})\]";
pub const REGEX_LOG_LEVEL: &str = r"\[(\binfo\b|\bwarn\b)\]";
pub const REGEX_APP_STRING: &str = r"App:";
// unix: 1667481423234
pub const REGEX_UNIX_NANOS: &str = r"(unix:)( )(\d{13})";
// bpm: [range 30-250]
pub const REGEX_BPM_HEART_RATE: &str = r"(bpm:)( )(\d{2,3})";


fn main() {
    let filename = "test.log";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    // Read the File to a buffered reader for efficiency
    let reader = BufReader::new(file);

    // Iterate the lines of the buffered file
    reader.lines().for_each(
        |line| {
            // Parse every line using a regex with capture groups
            println!("{}", line.unwrap());
        }
    )
}

/// Apply RegEx to a log file line, and read the capture groups into an array of strings.
// https://docs.rs/regex/latest/regex/#example-iterating-over-capture-groups
fn capture_group_for_line(re: Regex, line: &str) {
    let mut res = vec![];
    for cap in re.captures_iter(line.clone()) {
        println!("CAP_IDX_0:{}", &cap[0]);
        println!("CAP_IDX_1:{}", &cap[1]);
        println!("CAP_IDX_2:{}", &cap[2]);
        res.push(cap)
    }
}

fn extract_from_line(line: &str) -> Vec<&str> {
    let mut results = vec![];

    let re_time = Regex::new(REGEX_TIME).unwrap();
    if let Some(timestamp) = re_time.find(line.clone()) {
        results.push(timestamp.as_str())
    }

    let re_log_level = Regex::new(REGEX_LOG_LEVEL).unwrap();
    if let Some(log_level) = re_log_level.find(line.clone()) {
        results.push(log_level.as_str())
    }

    let re_app_str = Regex::new(REGEX_APP_STRING).unwrap();
    if let Some(app_str) = re_app_str.find(line.clone()) {
        results.push(app_str.as_str())
    }

    let re_unix_time = Regex::new(REGEX_UNIX_NANOS).unwrap();
    if let Some(unix_nano) = re_unix_time.find(line.clone()) {
        results.push(unix_nano.as_str())
    }

    let re_bpm = Regex::new(REGEX_BPM_HEART_RATE).unwrap();
    if let Some(bpm) = re_bpm.find(line.clone()) {
        results.push(bpm.as_str())
    }

    println!("{:#?}", results);
    results
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
