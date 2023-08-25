use std::process::Command;
mod acpi;

struct CommandSettings {
    critical_percent: u8,
    critical_notification_timeout: i32,
}

fn command(settings: CommandSettings) -> i32 {
    let critical_percent = settings.critical_percent;
    let acpi_result = Command::new("acpi").args(["-b"]).output();

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
                            .args([
                                "-u",
                                "critical",
                                "Warning: Battery",
                                &format!("Battery low {}%", percent),
                                "-t",
                                &settings.critical_notification_timeout.to_string(),
                            ])
                            .output()
                            .expect("Could not execute notify-send");
                    }
                    0
                }
                Err(_) => {
                    eprintln!("Could not understand acpi output");
                    1
                }
            }
        }
        Err(_) => {
            eprintln!("Could not execute acpi");
            1
        }
    }
}

fn main() -> ! {
    let status = command(CommandSettings {
        critical_percent: 15,
        critical_notification_timeout: 2000,
    });
    std::process::exit(status);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, path::Path};

    #[test]
    fn does_not_notify_when_normal() {
        let notify_send_out = ".notify-send_normal.out";
        fs::remove_file(notify_send_out).ok();
        env::set_var("PATH", "./test-bin/normal");

        let status = command(CommandSettings {
            critical_percent: 15,
            critical_notification_timeout: 2000,
        });
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

        let status = command(CommandSettings {
            critical_percent: 15,
            critical_notification_timeout: 2000,
        });
        assert_eq!(status, 0);

        let exists = Path::new(notify_send_out).exists();
        assert!(exists);
    }
}
