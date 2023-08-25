use std::{process::Command};



pub(crate) mod acpi {
    use std::fmt;

    use regex::Regex;

    pub(super) struct AcpiOutput {
        // battery: String,
        // status: String,
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
        let output_regex = Regex::new(r"Battery 0: .*, (?<percent>.*)%, .*").unwrap();
        match output_regex.captures(&output) {
            Some(captures) => {
                return Ok(AcpiOutput { 
                    // battery: String::from("Battery 0"),
                    // status: String::from("discharging"),
                    percent: captures.name("percent")
                        .unwrap()
                            .as_str()
                            .parse::<u8>()
                            .unwrap(), 
                    // remaining: String::from("10:00:20") 
                });
            },
            None => {
                Err(AcpiError)
            },
        }
    }
}

struct CommandSettings {
    critical_percent: u8,
    critical_notification_timeout: i32,
}

fn command(settings: CommandSettings) -> i32 {
    let critical_percent = settings.critical_percent;
    let acpi_result = Command::new("acpi")
        .args([ "-b", ])
        .output();

    match acpi_result {
        Ok(output) => {
            println!("acpi: {}", output.status);
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);

            match acpi::from_string(stdout) {
                Ok(acpi_output) => {
                    let percent = acpi_output.percent;
                    if percent < critical_percent {

                        Command::new("notify-send")
                            .args(["-u", "critical", "Warning: Battery", &format!("Battery low {}%", percent), "-t", &settings.critical_notification_timeout.to_string()])
                            .output()
                            .expect("Could not execute notify-send");
                    }
                    0
                },
                Err(_) => {
                    eprintln!("Could not understand acpi output");
                    1
                }
            }
        },
        Err(_) => {
            eprintln!("Could not execute acpi");
            1
        }
    }
}

fn main() -> ! {
    let status = command(CommandSettings{critical_percent: 15, critical_notification_timeout: 2000});
    std::process::exit(status);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path, env};


    #[test]
    fn does_not_notify_when_normal() {
        let notify_send_out = ".notify-send_normal.out";
        fs::remove_file(notify_send_out).ok();
        env::set_var("PATH", "./test-bin/normal");
        
        let status = command(CommandSettings{critical_percent: 15, critical_notification_timeout: 2000});
        assert_eq!(status, 0);

        let exists = Path::new(notify_send_out).exists();
        assert!(!exists);
    }

    #[test]
    fn notifies_critical_when_critical_low() {
        let notify_send_out = ".notify-send_critical-low.out";
        fs::remove_file(notify_send_out).ok();
        println!("Original PATH:{}", env::var("PATH").unwrap());
        env::set_var("PATH", "./test-bin/critical-low");
        println!("After PATH:{}", env::var("PATH").unwrap());
        
        let status = command(CommandSettings{critical_percent: 15, critical_notification_timeout: 2000});
        assert_eq!(status, 0);

        let exists = Path::new(notify_send_out).exists();
        assert!(exists);
    }

}

#[cfg(test)]
mod acpi_tests {
    use super::acpi;

    #[test]
    fn should_extract_percent_when_charging() {
        let output = acpi::from_string("Battery 0: Charging, 76%, 04:47:12 until charged".to_string())
            .expect("from_string failed");

        let expected_percent = 76;
        assert_eq!(output.percent, expected_percent, "Percent should be {} (was: {})", expected_percent, output.percent);
    }

    #[test]
    fn should_extract_percent_when_discharging() {
        let output = acpi::from_string("Battery 0: Discharging, 6%, 0:5:12 remaining".to_string())
            .expect("from_string failed");

        let expected_percent = 6;
        assert_eq!(output.percent, expected_percent, "Percent should be {} (was: {})", expected_percent, output.percent);
    }
}
