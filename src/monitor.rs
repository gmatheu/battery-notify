struct MonitorResponse {
    delay: i32,
    duration: i32,
}
type MonitorResponseGenerator = Box<dyn Fn() -> MonitorResponse>;
struct MonitorSettings {
    retries: i32,
    normal_delay: i32,
    response_generator: MonitorResponseGenerator,
}

struct Monitor {
    normal_delay: i32,
    pending: i32,
    response_generator: MonitorResponseGenerator,
}

impl Monitor {
    pub fn new(settings: Option<MonitorSettings>) -> Self {
        let _settings = settings.unwrap_or(MonitorSettings {
            retries: 10,
            normal_delay: 5000,
            response_generator: Box::new(|| MonitorResponse {
                delay: 5000,
                duration: 2000,
            }),
        });
        let retries = _settings.retries;
        let normal_delay = _settings.normal_delay;
        let response_generator = _settings.response_generator;
        Self {
            normal_delay,
            pending: retries,
            response_generator,
        }
    }

}

impl Default for Monitor {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Iterator for Monitor {
    type Item = MonitorResponse;

    fn next(&mut self) -> Option<Self::Item> {
        self.pending -= 1;
        if self.pending >= 0 {

            Some((self.response_generator)())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_duration() {
        let monitor = Monitor::new(Some(MonitorSettings {
            retries: 5,
            normal_delay: 1,
            response_generator: Box::new(|| MonitorResponse {
                duration: 1,
                delay: 1,
            }),
        }));

        let mut counter: i32 = 0;
        for response in monitor {
            counter += 1;
            assert_eq!(response.delay, 1);
            assert_eq!(response.duration, 1);
        }
        assert_eq!(counter, 5)
    }

    #[test]
    fn duration_can_be_modified() {
        let monitor = Monitor::new(Some(MonitorSettings {
            retries: 5,
            normal_delay: 1,
            response_generator: Box::new(|| MonitorResponse {
                duration: 1,
                delay: 1,
            }),
        }));

        let mut counter: i32 = 0;
        for response in monitor {
            counter += 1;
            assert_eq!(response.delay, 1);
            assert_eq!(response.duration, 1);
        }
        assert_eq!(counter, 5)
    }
}
