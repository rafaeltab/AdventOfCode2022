use crate::days::Day;

pub struct DayOne {

}

const TEST_PATH: &str = "./src/days/day_one/test.txt";
const DATA_PATH: &str = "./src/days/day_one/data.txt";

impl Day for DayOne {
	fn exec(&self, test: bool) -> () {
		let input_path: &str;

		if test {
			input_path = TEST_PATH;
		} else {
			input_path = DATA_PATH;
		}

		let data = std::fs::read_to_string(input_path).expect("Could not open file");

		let parsed_data = parse_data(&data);

		let sums = calulcate_sums(&parsed_data);

		let max = find_max(&sums);

		println!("Max: {}", max);

	}
}

struct DayOneData {
	elves: Vec<Vec<i32>>
}

fn parse_data(data: &str) -> DayOneData {
	let split = data.split("\r\n\r\n");

	let mut data = DayOneData {
		elves: Vec::new()
	};

	for elf in split {
		let mut elf_data = Vec::new();

		elf.lines().for_each(|line| {
			let parsed = line.parse::<i32>();
			if !parsed.is_err() {
				elf_data.push(parsed.unwrap());
			}
		});

		data.elves.push(elf_data);
	}

	return data;
}

fn calulcate_sums(data: &DayOneData) -> Vec<i32> {
	let mut sums = Vec::new();

	for elf in data.elves.iter() {
		let mut sum = 0;
		for calories in elf {
			sum += calories;
		}
		sums.push(sum);
	}

	return sums;
}

fn find_max(elves: &Vec<i32>) -> i32 {
	let mut max = 0;

	for elf in elves.iter() {
		if elf > &max {
			max = *elf;
		}
	}

	return max;
}