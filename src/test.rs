#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use regex::Regex;
    use crate::{capture_group_for_line, extract_from_line};
    use crate::regex::REGEX_TIME;

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
