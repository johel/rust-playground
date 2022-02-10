#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

pub fn to_minutes(hours: i32, minutes: i32) -> u32 {
    let minute = (hours * 60 + minutes) % 60;
    let unsigned_minute = if minute >= 0 { minute } else { minute + 60 };
    unsigned_minute.try_into().unwrap()
}

pub fn to_hours(hours: i32, minutes: i32) -> u32 {
    let hours = hours as f64;
    let minutes = minutes as f64;
    let hour = hours + minutes / 60.0;
    let mut signed_hour = hour.floor() as i32;
    signed_hour = signed_hour % 24;
    let unsigned_hour = if signed_hour >= 0 {
        signed_hour
    } else {
        signed_hour + 24
    };
    unsigned_hour.try_into().unwrap()
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes, hours }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: self.minutes + minutes,
            hours: self.hours,
        }
    }

    pub fn to_string(&self) -> String {
        let hours = to_hours(self.hours, self.minutes);
        let minutes = to_minutes(self.hours, self.minutes);
        let formated_hours = if hours < 10 {
            format!("0{}", hours)
        } else {
            format!("{}", hours)
        };
        let formated_minutes = if minutes < 10 {
            format!("0{}", minutes)
        } else {
            format!("{}", minutes)
        };
        format!("{}:{}", formated_hours, formated_minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::to_hours;
    use crate::to_minutes;
    use crate::Clock;
    #[test]
    fn it_works() {
        let result = "19:59";
        assert_eq!(
            result,
            Clock {
                hours: -2,
                minutes: -121
            }
            .to_string()
        );
    }

    #[test]
    fn it_workss() {
        let result = to_hours(24, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_2() {
        let result = to_hours(-2, 40);
        assert_eq!(result, 22);
    }

    #[test]
    fn it_works_3() {
        let result = -2;
        let calc = -1.2f64.ceil() as i32;
        assert_eq!(result, calc % 24);
    }

    #[test]
    fn it_works_4() {
        let result = to_hours(-1, -20);
        assert_eq!(result, 22);
    }
}
