use std::process::Command;

#[derive(Debug)]
pub(crate) struct CommandSettings {
    pub(super) critical_percent: u8,
    pub(super) critical_notification_timeout: i32,
}
pub(crate) const DEFAULT_COMMAND_SETTINGS: CommandSettings = CommandSettings {
    critical_percent: 15,
    critical_notification_timeout: 10_000,
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
        self.notify(message, NotificationLevel::Critical, duration);
    }
}

pub(crate) struct SendNotify {
    title: &'static str,
}
impl SendNotify {
    fn execute(title: &str, level: &str, message: &str, duration: i32) {
        Command::new("notify-send")
            .args(["-u", level, title, message, "-t", &duration.to_string()])
            .output()
            .expect("Could not execute notify-send");
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
                SendNotify::execute(self.title, message, "critical", duration);
            }
        }
    }
}
