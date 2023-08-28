use std::process::Command;

use notifier::{CommandSettings, SendNotify, Notifier};
mod acpi;
mod notifier;



fn command(optional_settings: Option<CommandSettings>) -> i32 {
    let settings = optional_settings.unwrap_or_else(|| notifier::DEFAULT_COMMAND_SETTINGS);
    let critical_percent = settings.critical_percent;
    let acpi_result = Command::new("acpi").args(["-b"]).output();

    let notifier: SendNotify = Notifier::new("Warning: Battery");
    match acpi_result {
        Ok(output) => {
            println!("acpi: {}", output.status);
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);

            match acpi::from_string(stdout) {
                Ok(acpi_output) => {
                    let percent = acpi_output.percent;
                    if percent < critical_percent {
                        notifier.notify_critical(&format!("Battery low {}%", percent), settings.critical_notification_timeout)
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
    let status = command(None);
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

        let status = command(None);
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

        let status = command(None);
        assert_eq!(status, 0);

        let exists = Path::new(notify_send_out).exists();
        assert!(exists);
    }
}
