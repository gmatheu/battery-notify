use std::process::Command;

#[derive(Debug)]
pub(crate) struct CommandSettings {
    pub(super) critical_percent: u8,
    pub(super) critical_notification_timeout: i32,
}
pub(crate) const DEFAULT_COMMAND_SETTINGS: CommandSettings = CommandSettings {
    critical_percent: 15,
    critical_notification_timeout: 2_000,
};

pub(crate) enum NotificationLevel {
    Low,
    Normal,
    Critical,
}
pub(crate) trait Notifier {
    fn new(title: &'static str) -> Self;

    fn notify(&self, message: &str, level: NotificationLevel, duration: i32);

    fn notify_critical(&self, message: &str, duration: i32) {
        println!("Sending critical notification: {duration}");
        self.notify(message, NotificationLevel::Critical, duration);
    }
}

pub(crate) struct SendNotify {
    title: &'static str,
}
impl SendNotify {
    fn execute(title: &str, level: &str, message: &str, duration: i32) {
        let output = Command::new("notify-send")
            .args(["-u", level, title, message, "-t", &duration.to_string()])
            .output();
        match output {
            Ok(stdout) => {
                println!("send-notify executed");
                println!("{}", String::from_utf8(stdout.stdout).unwrap());
                println!("{}", String::from_utf8(stdout.stderr).unwrap());

            },
            Err(_) => println!("Could not execute send-notify"),
        }
    }
}
impl Notifier for SendNotify {
    fn new(title: &'static str) -> SendNotify {
        SendNotify { title }
    }

    fn notify(&self, message: &str, level: NotificationLevel, duration: i32) {
        match level {
            NotificationLevel::Low => todo!(),
            NotificationLevel::Normal => todo!(),
            NotificationLevel::Critical => {
                SendNotify::execute(self.title, "critical", message , duration);
            }
        }
    }
}
