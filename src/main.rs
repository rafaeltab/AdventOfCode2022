pub mod days;

struct Arguments {
    day: i32,
    test: bool,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut arguments = Arguments {
        day: 1,
        test: false,
    };

    for arg in args {
        println!("{}", arg);
        if arg == "--test" {
            arguments.test = true;
        } else {
            let parsed = arg.parse::<i32>();

            if !parsed.is_err() {
                arguments.day = parsed.unwrap();
            }
        }
    }

    let day = days::get_day(&arguments.day);

    println!("{} {}", "Starting AoC day", arguments.day);

    day.exec(arguments.test);
}
