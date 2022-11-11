/* This project is released under:
 *
 * "THE BEER-WARE LICENSE" (Revision 42):
 *
 * As long as you retain this notice you can do whatever you want
 * with this stuff. If we meet some day, and you think this stuff
 * is worth it, you can buy me a beer in return.
 *
 * This project is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY;  without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
*/
use std::{
	env,      /* for args.collect() */
	process,  /* for exit() */
};

const CANCEL_OPERATION: i32 = 125;  /* exit code */

fn print_help() {
	eprintln!("Usage: meantime [-d<elimiter>] HH:MM:SS hh:mm:ss");
}

fn exerr(message: &str) {
	eprintln!("Error: {}", message);
	print_help();
	process::exit(CANCEL_OPERATION);
}

fn print_result(hour: u32, minute: u32, second: u32) {
	println!("{:02}:{:02}:{:02}", hour, minute, second);
}

/* Convert string-slice number to actual unsigned int number.
 * If there is unexpected value, throw the error information
 * and stop the program.
 */

fn hour_to_second(hour: &str) -> u32 {
	match hour.parse::<u32>() {
		Ok(valid)  => return 3600 * valid,
		Err(error) => {
			exerr(&format!("{error}"));
			return 0;
		}
	}
}

fn minute_to_second(minute: &str) -> u32 {
	match minute.parse::<u32>() {
		Ok(valid)  => return valid * 60,
		Err(error) => {
			exerr(&format!("{error}"));
			return 0;
		}
	}
}

fn second_to_second(second: &str) -> u32 {
	match second.parse::<u32>() {
		Ok(valid)  => return valid,
		Err(error) => {
			exerr(&format!("{error}"));
			return 0;
		}
	}
}

fn second_to_hour(second: &mut u32) -> u32 {
	let mut count: u32 = 0;

	while *second >= 3600 {
		*second -= 3600;
		count   += 1;
	}

	return count;
}

fn second_to_minute(second: &mut u32) -> u32 {
	let mut count: u32 = 0;

	while *second >= 60 {
		*second -= 60;
		count   += 1;
	}

	return count;
}

fn count_the_time(arg_time: &Vec<&str>, len: &usize) -> u32 {
	let mut sec_time = 0;

	match len {
		3 => {
			sec_time += hour_to_second(arg_time[2]);
			sec_time += minute_to_second(arg_time[1]);
			sec_time += second_to_second(arg_time[0]);
		},
		2 => {
			sec_time += minute_to_second(arg_time[1]);
			sec_time += second_to_second(arg_time[0]);
		},
		1 => {
			sec_time += second_to_second(arg_time[0]);
		},
		_ => exerr("invalid time format."),
	}

	return sec_time;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let leng: usize       = args.len();

	let mut index: usize  = 1;    /* index of args, default is 1. */
	let mut delim: &str   = ":";  /* default delimiter is ':'. */

	if leng == 4 {
		/* user specify delimiter. Now index start from 2. */
		match args[index].strip_prefix("-d") {
			Some(dl) => delim = dl,
			None     => exerr(&format!("unknown option {}", args[index])),
		}
		index += 1;
	} else {
		if leng != 3 {
			exerr("expecting more or less argument");
		}
	}

	let arg_time1: Vec<&str> = args[index].rsplit(delim).collect();
	let len_time1: usize     = arg_time1.len();
	let mut sec_time1: u32   = 0;

	index += 1;

	let arg_time2: Vec<&str> = args[index].rsplit(delim).collect();
	let len_time2: usize     = arg_time2.len();
	let mut sec_time2: u32   = 0;

	sec_time1 += count_the_time(&arg_time1, &len_time1);
	sec_time2 += count_the_time(&arg_time2, &len_time2);

	let mut sec_result: u32  = 0;

	// Flexibility.
	if sec_time1 >= sec_time2 {
		sec_result += sec_time1 - sec_time2;
	} else {
		sec_result += sec_time2 - sec_time1;
	}

	print_result(second_to_hour(&mut sec_result),
		second_to_minute(&mut sec_result), sec_result
	);
}
// vim:ts=2:sw=2:noexpandtab:cindent
