use std::fmt;

const DAY_IN_MINUTES: i32 = 1440;
const HOUR_IN_MINUTES: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * HOUR_IN_MINUTES + minutes;
        Clock {
            minutes: Self::to_valid_minutes(total_minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Self::to_valid_minutes(self.minutes + minutes),
        }
    }

    fn to_valid_minutes(minutes: i32) -> i32 {
        if minutes >= 0 {
            minutes % DAY_IN_MINUTES
        } else {
            DAY_IN_MINUTES + (minutes % DAY_IN_MINUTES)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / HOUR_IN_MINUTES;
        let minutes = self.minutes % HOUR_IN_MINUTES;
        write!(formatter, "{:02}:{:02}", hours, minutes)
    }
}
