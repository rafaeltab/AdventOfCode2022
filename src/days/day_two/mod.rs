use crate::days::Day;

pub struct DayTwo {}

const TEST_PATH: &str = "./src/days/day_two/test.txt";
const DATA_PATH: &str = "./src/days/day_two/data.txt";

impl Day for DayTwo {
    fn exec(&self, test: bool) -> () {
        let input_path: &str;

        if test {
            input_path = TEST_PATH;
        } else {
            input_path = DATA_PATH;
        }

        let data = std::fs::read_to_string(input_path).expect("Could not open file");

        let parsed_data = parse_data(&data);

        let step_one_score = simulate_step_one(&parsed_data);
        let step_two_score = simulate_step_two(&parsed_data);

        println!("Step one score: {}", step_one_score);
        println!("Step two score: {}", step_two_score);
    }
}

enum RPCMove {
    Rock,
    Paper,
    Scissors,
}

enum RPCResult {
    Win,
    Draw,
    Loss,
}

struct DayTwoData {
    matches: Vec<(RPCMove, RPCMove)>,
}

fn parse_data(data: &str) -> DayTwoData {
    let mut ret_data = DayTwoData {
        matches: Vec::new(),
    };

    for m in data.lines() {
        let split: Vec<&str> = m.split(" ").collect();
        let theirs = str_to_move(split[0]);
        let ours = str_to_move(split[1]);
        ret_data.matches.push((theirs, ours))
    }

    return ret_data;
}

fn str_to_move(m: &str) -> RPCMove {
    match m {
        "A" => RPCMove::Rock,
        "B" => RPCMove::Paper,
        "C" => RPCMove::Scissors,
        "X" => RPCMove::Rock,
        "Y" => RPCMove::Paper,
        "Z" => RPCMove::Scissors,
        _ => panic!("Invalid move"),
    }
}

fn move_to_result(m: &RPCMove) -> RPCResult {
    return match m {
        RPCMove::Rock => RPCResult::Loss,
        RPCMove::Paper => RPCResult::Draw,
        RPCMove::Scissors => RPCResult::Win,
    };
}

fn simulate_step_one(data: &DayTwoData) -> i32 {
    let mut score = 0;

    for m in data.matches.iter() {
        score += simulate_round(&m.0, &m.1);
    }

    return score;
}

fn simulate_step_two(data: &DayTwoData) -> i32 {
    let mut score = 0;

    for m in data.matches.iter() {
        let mut round_score = 0;

        let our_move: RPCMove = match move_to_result(&m.1) {
            RPCResult::Win => match m.0 {
                RPCMove::Rock => RPCMove::Paper,
                RPCMove::Paper => RPCMove::Scissors,
                RPCMove::Scissors => RPCMove::Rock,
            },
            RPCResult::Draw => match m.0 {
                RPCMove::Rock => RPCMove::Rock,
                RPCMove::Paper => RPCMove::Paper,
                RPCMove::Scissors => RPCMove::Scissors,
            },
            RPCResult::Loss => match m.0 {
                RPCMove::Rock => RPCMove::Scissors,
                RPCMove::Paper => RPCMove::Rock,
                RPCMove::Scissors => RPCMove::Paper,
            },
        };

		round_score += simulate_round(&m.0, &our_move);

        score += round_score;
    }

    return score;
}

fn simulate_round(their_move: &RPCMove, our_move: &RPCMove) -> i32 {
    let mut round_score = 0;

    round_score += match our_move {
        RPCMove::Rock => 1,
        RPCMove::Paper => 2,
        RPCMove::Scissors => 3,
    };

    round_score += match our_move {
        RPCMove::Rock => match their_move {
            RPCMove::Rock => 3,
            RPCMove::Paper => 0,
            RPCMove::Scissors => 6,
        },
        RPCMove::Paper => match their_move {
            RPCMove::Rock => 6,
            RPCMove::Paper => 3,
            RPCMove::Scissors => 0,
        },
        RPCMove::Scissors => match their_move {
            RPCMove::Rock => 0,
            RPCMove::Paper => 6,
            RPCMove::Scissors => 3,
        },
    };
    return round_score;
}
