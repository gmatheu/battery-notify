use regex::Regex;
use std::{process::Command, fmt::Error};

fn main() -> () {
    let acpi_result = Command::new("acpi")
        .args([ "-b", ])
        .output();

    struct AcpiOutput {
        battery: String,
        status: String,
        percent: i8,
        remaining: String
    }

    fn from_string(output: String) -> Result<AcpiOutput, Error> {
        let re = Regex::new(r"Battery (.): (.*), (.*)%, (.*) remaining").unwrap();
        return Ok(AcpiOutput { battery: String::from("Battery 0"), status: String::from("discharging"), percent: 50, remaining: String::from("10:00:20") });
    }
    match acpi_result {
        Ok(output) => {
            println!("acpi: {}", output.status);
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}", stdout);

            let acpi_output = from_string(stdout);
            let percent = 10;
            Command::new("notify-send")
                .args(["-u", "critical", "Warning: Battery", &format!("Battery low {}%", percent), "-t", "2000"])
                .output()
                .expect("Could not execute acpi");
        },
        Err(_) => {
            panic!("Could not execute acpi");
        }
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}
