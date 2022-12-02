pub mod day_one;
pub mod day_two;

pub trait Day {
    fn exec(&self, test: bool) -> ();
}

pub fn get_day(nr: &i32) -> Box<dyn Day> {
    match nr {
        1 => Box::new(day_one::DayOne {}),
        2 => Box::new(day_two::DayTwo {}),
        _ => panic!("Day not implemented"),
    }
}
