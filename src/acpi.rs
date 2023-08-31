use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Unknown,
}
pub(super) struct AcpiOutput {
    // battery: String,
    pub status: BatteryStatus,
    pub percent: u8,
    // remaining: String
}

pub(super) struct AcpiError;
// Implement std::fmt::Display for AppError
impl fmt::Display for AcpiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AcpiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub(crate) fn from_string(output: String) -> Result<AcpiOutput, AcpiError> {
    // let output_regex = Regex::new(r"Battery (.): (.*), (?<percent>..*)%, (.*) (.*)").unwrap();
    // "Battery (?<battery>.): (?<status>.*), (?<percent>..*)%, (?<eta>[\d:]*) (?<target>.*)";
    let output_regex = Regex::new(r"Battery 0: (?<status>.*), (?<percent>.*)%, .*").unwrap();
    match output_regex.captures(&output) {
        Some(captures) => {
            return Ok(AcpiOutput {
                // battery: String::from("Battery 0"),
                status: from(captures.name("status").unwrap().as_str()),
                percent: captures
                    .name("percent")
                    .unwrap()
                    .as_str()
                    .parse::<u8>()
                    .unwrap(),
                // remaining: String::from("10:00:20")
            });
        }
        None => Err(AcpiError),
    }
}

fn from(as_str: &str) -> BatteryStatus {
    match as_str {
        "Charging" => BatteryStatus::Charging,
        "Discharging" => BatteryStatus::Discharging,
        _ => BatteryStatus::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use crate::acpi::{from_string, BatteryStatus};
    #[test]
    fn should_extract_percent_when_charging() {
        let output = from_string("Battery 0: Charging, 76%, 04:47:12 until charged".to_string())
            .expect("from_string failed");

        let expected_percent = 76;
        assert_eq!(
            output.percent, expected_percent,
            "Percent should be {} (was: {})",
            expected_percent, output.percent
        );
        assert_eq!(output.status, BatteryStatus::Charging);
    }

    #[test]
    fn should_extract_percent_when_discharging() {
        let output = from_string("Battery 0: Discharging, 6%, 0:5:12 remaining".to_string())
            .expect("from_string failed");

        let expected_percent = 6;
        assert_eq!(
            output.percent, expected_percent,
            "Percent should be {} (was: {})",
            expected_percent, output.percent
        );
        assert_eq!(output.status, BatteryStatus::Discharging);
    }
}
