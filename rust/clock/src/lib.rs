#[derive(PartialEq, Debug)]
pub struct Clock{
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock{ minutes: 0 };
        clock.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let minutes_in_clock = (new_minutes % 1440 + 1440 ) % 1440;
        Clock{ minutes: minutes_in_clock }
    }

    pub fn to_string(&self) -> String{
        let hours = self.minutes / 60 ;
        let minutes = self.minutes % 60 ;
        format!("{:0>2}:{:0>2}", hours, minutes)
    }
}
