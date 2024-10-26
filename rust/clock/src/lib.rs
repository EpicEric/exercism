use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn calculate_time(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes;
        let mut hours = hours;
        while minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        while minutes >= 60 {
            hours += 1;
            minutes -= 60;
        }
        while hours < 0 {
            hours += 24;
        }
        Self {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::calculate_time(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::calculate_time(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }
}
