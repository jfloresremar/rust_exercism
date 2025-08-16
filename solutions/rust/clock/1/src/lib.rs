use std::fmt::Display;

#[derive(Debug,PartialEq)] 
pub struct Clock{
    time_in_minutes: i32
}

impl Clock {

    // pub fn new(hours: i32, minutes: i32) -> Self {
    //     //todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
    //     let mut _minutes=minutes%60;
    //     let mut _hours=(hours+minutes/60)%24;
    //     if minutes<0 {
    //         _minutes=60+_minutes;
    //     }
    //     if hours<0{
    //         _hours=24+_hours;
    //     }

    //     Self { hours: _hours, minutes: _minutes }
    // }
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self{
            time_in_minutes:(1440+((hours*60+minutes)%1440))%1440
        }

    }

    // pub fn add_minutes(&self, minutes: i32) -> Self {
    //     //todo!("Add {minutes} minutes to existing Clock time");
    //     let mut _minutes=(self.minutes+minutes)%60;
    //     let mut _hours=(self.hours+(self.minutes+minutes)/60)%24;
    //     if minutes<0 {
    //         _minutes=60+_minutes;
    //     }
    //     Self { hours: _hours, minutes: _minutes }
    // }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self{
            time_in_minutes:(1440+((self.time_in_minutes+minutes)%1440))%1440
        }

    }
}

impl Display for Clock{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:02}:{:02}",self.time_in_minutes/60,self.time_in_minutes%60)
    }
}