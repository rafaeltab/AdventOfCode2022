pub mod day_one;

pub trait Day {
    fn exec(&self, test: bool) -> ();
}

pub fn get_day(nr: &i32) -> Box<dyn Day> {
    match nr {
        1 => Box::new(day_one::DayOne {}),
        _ => panic!("Day not implemented"),
    }
}
