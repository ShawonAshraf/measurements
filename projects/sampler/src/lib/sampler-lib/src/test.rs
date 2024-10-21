#[cfg(test)]
mod test {
    use crate::types::Measurement;
    use crate::utils::*;
    use chrono::prelude::*;

    #[test]
    fn test_get_interval_start() {
        let dt = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let start = get_interval_start(dt);
        assert_eq!(start, Utc.ymd(2021, 1, 1).and_hms(0, 0, 0));
    }

    #[test]
    fn test_str_to_utc() {
        let ts = "2021-01-01T00:00:00".to_string();
        let dt = str_to_utc(&ts);
        assert_eq!(dt, Utc.ymd(2021, 1, 1).and_hms(0, 0, 0));
    }

    #[test]
    fn test_utc_to_str() {
        let dt = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let ts = utc_to_str(&dt);
        assert_eq!(ts, "2021-01-01T00:00:00");
    }

    #[test]
    fn test_round_timestamp_to_boundary() {
        let ts = "2021-01-01T00:00:00".to_string();
        let rounded = round_timestamp_to_boundary(&ts);
        assert_eq!(rounded, "2021-01-01T00:00:00");

        let ts = "2021-01-01T00:02:00".to_string();
        let rounded = round_timestamp_to_boundary(&ts);
        assert_eq!(rounded, "2021-01-01T00:05:00");
    }

    #[test]
    fn test_find_latest_in_interval() {
        let measurement1 = Measurement {
            timestamp: String::from("2017-01-03T10:04:45"),
            measurement_type: String::from("TEMP"),
            value: 35.79,
        };
        let measurement2 = Measurement {
            timestamp: String::from("2017-01-03T10:01:18"),
            measurement_type: String::from("SPO2"),
            value: 98.78,
        };
        let measurement3 = Measurement {
            timestamp: String::from("2017-01-03T10:09:07"),
            measurement_type: String::from("TEMP"),
            value: 35.01,
        };
        let measurement4 = Measurement {
            timestamp: String::from("2017-01-03T10:03:34"),
            measurement_type: String::from("SPO2"),
            value: 96.49,
        };
        let measurement5 = Measurement {
            timestamp: String::from("2017-01-03T10:02:01"),
            measurement_type: String::from("TEMP"),
            value: 35.82,
        };
        let measurement6 = Measurement {
            timestamp: String::from("2017-01-03T10:05:00"),
            measurement_type: String::from("SPO2"),
            value: 97.17,
        };
        let measurement7 = Measurement {
            timestamp: String::from("2017-01-03T10:05:01"),
            measurement_type: String::from("SPO2"),
            value: 95.08,
        };

        let test_data: Vec<&Measurement> = vec![
            &measurement1,
            &measurement2,
            &measurement3,
            &measurement4,
            &measurement5,
            &measurement6,
            &measurement7,
        ];

        let latest = find_latest_in_interval(&test_data);
        assert_eq!(latest.len(), 2);
    }
}
