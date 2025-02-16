use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours % 24;
        let mut clock = Self { hours, minutes: 0 };
        clock.add_minutes(minutes)
    }

    fn get_minutes_and_hours(minutes: i32) -> (i32, i32) {
        if minutes < 0 {
            let clock = Self::get_minutes_and_hours(60 + minutes);
            return (clock.0 - 1, clock.1);
        }
        let hours = minutes / 60;
        let minutes = minutes % 60;
        (hours, minutes)
    }

    fn raise_hours_to_positive(&self) -> Clock {
        let mut hours = self.hours;
        while hours < 0 {
            hours += 24;
        }
        Clock { hours, minutes: self.minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let clock = self.raise_hours_to_positive();
        let mut minutes = clock.minutes + minutes;

        let calc_min_and_hours = Self::get_minutes_and_hours(minutes);
        minutes = calc_min_and_hours.1;
        hours = clock.hours + calc_min_and_hours.0;
        let clock = Clock { hours, minutes };

        let clock = clock.raise_hours_to_positive();
        hours = clock.hours % 24;

        Self { hours, minutes: clock.minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}