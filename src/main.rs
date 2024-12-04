#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]
use core::panic;
use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use std::env::args;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use regex::Regex;
use ndarray::{Array, Array1, Array2, Axis};

fn read_lines(day: u8) -> impl Iterator<Item = Result<String, impl std::error::Error>> {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = format!("{}/inputs/{}", cargo_dir, day);
    let Ok(file) = File::open(&dir) else {
        panic!("File for day {day} not found at {dir}");
    };
    BufReader::new(file).lines().into_iter()
}

fn day_1() {
    let inputs = read_lines(1);
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in inputs {
        let temp_vec: Vec<i32> = line.unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        list1.push(temp_vec[0]);
        list2.push(temp_vec[1]);
    }
    list1.sort();
    list2.sort();

    let mut diff_list: Vec<i32> = Vec::new();
    for (num1, num2) in list1.iter().zip(list2.iter()) {
        diff_list.push((num1 - num2).abs());        
    }
    let result1: i32 = diff_list.iter().sum();

    let  mut similarity: i32 = 0;
    for number in list1 {
        let occurences: i32 = list2.iter().filter(|x| **x==number).count().try_into().unwrap();
        similarity += occurences * number;
    }
    println!("Results for day 1: {} and {}", result1, similarity);
}

fn check_report(input_vec: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing: bool = true;
    let mut proper_dist: bool = true;
    for (index, _) in input_vec.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if input_vec[index] < input_vec[index - 1] {
            increasing = false;
        }
        if input_vec[index] > input_vec[index - 1] {
            decreasing = false;
        }
        if (input_vec[index] - input_vec[index - 1]).abs() < 1 || (input_vec[index] - input_vec[index - 1]).abs() > 3 {
            proper_dist = false;
        }
    }
    if (increasing || decreasing) & proper_dist {
        return true;
    }
    return false;
}

fn day_2() {
    let inputs = read_lines(2);
    let mut nreports_save: u32 = 0;
    let mut nreports_save2: u32 = 0;
    for line in inputs {
        // PART ONE
        let row_vec: Vec<i32> = line.unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        if check_report(&row_vec) {
            nreports_save += 1;
        }
        // Part TWO
        if check_report(&row_vec) {
            nreports_save2 += 1;
            continue;
        }
        for i in 0..row_vec.len() {
        let temp_vec: Vec<i32> = row_vec.iter()
            .enumerate()
            .filter(|&(index,_)| index != i)
            .map(|(_, &val)| val)
            .collect();
            if check_report(&temp_vec) {
                nreports_save2 += 1;
                break;
            }
        }
    }
    println!("Results for day 2: {} and {}", nreports_save, nreports_save2);
}

fn day_3() {
    let inputs = read_lines(3);
    let re1 = Regex::new(r"mul\(\s*(-?\d+\.?\d*)\s*,\s*(-?\d+\.?\d*)\s*\)").unwrap();

    //let mut extracted_strings: Vec<String> = Vec::new();
    //for line in inputs {
    //    for hit in re1.captures_iter(line.unwrap().as_str()) {
    //        let matched: String = String::from(&hit[0]);
    //        extracted_strings.push(matched.replace("mul(", "").replace(")", ""));
    //    }
    //}
    //let formated: Vec<Vec<i32>> = extracted_strings.iter() 
    //            .map(|x| x.split(",")
    //                .map(|x| x.parse::<i32>()
    //                .unwrap()).collect::<Vec<_>>())
    //            .collect();
    //let result_part1: i32 = formated.iter().map(|x| x[0]*x[1]).sum();
    //println!("{:?}", result_part1);

    let re2 = Regex::new(r"(?P<instruction>do\(\)|don't\(\)|mul\(\s*-?\d+\.?\d*\s*,\s*-?\d+\.?\d*\s*\))").unwrap();
    let mut mul_enabled = true;
    let mut extracted_muls = Vec::new();
    for line in inputs {
        for caps in re2.captures_iter(line.unwrap().as_str()) {
            let instruction = &caps["instruction"];

            if instruction == "do()" {
                mul_enabled = true;
            } else if instruction == "don't()" {
                mul_enabled = false;                
            } else if instruction.starts_with("mul") {
                if mul_enabled {
                    let cleaned = instruction.replace("mul(", "").replace(")", "");
                    extracted_muls.push(cleaned);
                }
            }
        }
    }
    let formated: Vec<Vec<i32>> = extracted_muls.iter() 
                .map(|x| x.split(",")
                    .map(|x| x.parse::<i32>()
                    .unwrap()).collect::<Vec<_>>())
                .collect();
    let result_part2: i32 = formated.iter().map(|x| x[0]*x[1]).sum();
    println!("{:?}", result_part2);
    
}

fn diagonal_tuples(n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut diagonals = Vec::new();

    // Below and including the main diagonal
    for offset in 0..n {
        let mut diag = Vec::new();
        for i in 0..(n - offset) {
            diag.push((i + offset, i)); // (row, column)
        }
        diagonals.push(diag);
    }

    // Above the main diagonal
    for offset in 1..n {
        let mut diag = Vec::new();
        for i in 0..(n - offset) {
            diag.push((i, i + offset)); // (row, column)
        }
        diagonals.push(diag);
    }

    diagonals
}

fn reverse_diagonal_tuples(n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut diagonals = Vec::new();

    // Reverse diagonals starting from the first row
    for start_col in 0..n {
        let mut diag = Vec::new();
        let mut row = 0;
        let mut col = start_col;
        while row < n && col < n {
            diag.push((row, col)); // (row, column)
            row += 1;
            col = col.wrapping_sub(1); // Decrease column index
        }
        diagonals.push(diag);
    }

    // Reverse diagonals starting from the first column (excluding the top-right corner diagonal)
    for start_row in 1..n {
        let mut diag = Vec::new();
        let mut row = start_row;
        let mut col = n - 1;
        while row < n && col < n {
            diag.push((row, col)); // (row, column)
            row += 1;
            col = col.wrapping_sub(1); // Decrease column index
        }
        diagonals.push(diag);
    }

    diagonals
}

fn day_4() {
    let inputs = read_lines(4);
    let inputs_vec: Vec<String> = inputs.map(|x| x.unwrap()).collect();
    let columns: usize = inputs_vec[0].chars().count();  
    let rows: usize = inputs_vec.len();
    println!("{}, {}", rows, columns);

    let mut char_array: Array2<char> = Array::from_elem((rows, columns), '\0');

    // Populate array
    for row in 0..rows {
        let row_chars: Vec<char> = inputs_vec[row].chars().collect();
        for col in 0..columns {
            char_array[[row, col]] = row_chars[col];
        }
    }

    let mut count: usize = 0;
    // Through rows, backwards and forwards
    for row in 0..columns {
        let string_to_check: String = char_array.index_axis(Axis(0), row).iter().collect();
        count += string_to_check.matches("XMAS").count();
        count += string_to_check.matches("SAMX").count();
    } 

    // Top down and down up
    let char_array_T: Array2<char> = char_array.t().to_owned();
    for row in 0..columns {
        let string_to_check: String = char_array_T.index_axis(Axis(0), row).iter().collect();
        count += string_to_check.matches("XMAS").count();
        count += string_to_check.matches("SAMX").count();
    } 

    // diagonals
    let diagonal_indices = diagonal_tuples(columns);
    for diagonal in diagonal_indices {
        let mut string_to_check: String = String::new();
        for (row, col) in diagonal {
            string_to_check.push_str(&char_array[[row, col]].to_string());
        }
        count += string_to_check.matches("XMAS").count();
        count += string_to_check.matches("SAMX").count();
    }

    // reverse diagonals
    let reverse_diagonal_indices = reverse_diagonal_tuples(columns);
    for diagonal in reverse_diagonal_indices {
        let mut string_to_check: String = String::new();
        for (row, col) in diagonal {
            string_to_check.push_str(&char_array[[row, col]].to_string());
        }
        count += string_to_check.matches("XMAS").count();
        count += string_to_check.matches("SAMX").count();
    }


    println!("{}", count);
}

fn day_5() {
    println!("Result for day 5: {}", 0);
    let inputs = read_lines(5);
}

fn day_6() {
    println!("Result for day 6: {}", 0);
    let inputs = read_lines(6);
}

fn day_7() {
    println!("Result for day 7: {}", 0);
    let inputs = read_lines(7);
}

fn day_8() {
    println!("Result for day 8: {}", 0);
    let inputs = read_lines(8);
}

fn day_9() {
    println!("Result for day 9: {}", 0);
    let inputs = read_lines(9);
}

fn day_10() {
    println!("Result for day 10: {}", 0);
    let inputs = read_lines(10);
}

fn day_11() {
    println!("Result for day 11: {}", 0);
    let inputs = read_lines(11);
}

fn day_12() {
    println!("Result for day 12: {}", 0);
    let inputs = read_lines(12);
}

fn day_13() {
    println!("Result for day 13: {}", 0);
    let inputs = read_lines(13);
}

fn day_14() {
    println!("Result for day 14: {}", 0);
    let inputs = read_lines(14);
}

fn day_15() {
    println!("Result for day 15: {}", 0);
    let inputs = read_lines(15);
}

fn day_16() {
    println!("Result for day 16: {}", 0);
    let inputs = read_lines(16);
}

fn day_17() {
    println!("Result for day 17: {}", 0);
    let inputs = read_lines(17);
}

fn day_18() {
    println!("Result for day 18: {}", 0);
    let inputs = read_lines(18);
}

fn day_19() {
    println!("Result for day 19: {}", 0);
    let inputs = read_lines(19);
}

fn day_20() {
    println!("Result for day 20: {}", 0);
    let inputs = read_lines(20);
}

fn day_21() {
    println!("Result for day 21: {}", 0);
    let inputs = read_lines(21);
}

fn day_22() {
    println!("Result for day 22: {}", 0);
    let inputs = read_lines(22);
}

fn day_23() {
    println!("Result for day 23: {}", 0);
    let inputs = read_lines(23);
}

fn day_24() {
    println!("Result for day 24: {}", 0);
    let inputs = read_lines(24);
}

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].parse() {
        Ok(day) if (1u8..=24).contains(&day) => {
            println!("Running impl of day {} challenge..", { day });
            match day {
                1 => day_1(),
                2 => day_2(),
                3 => day_3(),
                4 => day_4(),
                5 => day_5(),
                6 => day_6(),
                7 => day_7(),
                8 => day_8(),
                9 => day_9(),
                10 => day_10(),
                11 => day_11(),
                12 => day_12(),
                13 => day_13(),
                14 => day_14(),
                15 => day_15(),
                16 => day_16(),
                17 => day_17(),
                18 => day_18(),
                19 => day_19(),
                20 => day_20(),
                21 => day_21(),
                22 => day_22(),
                23 => day_23(),
                24 => day_24(),
                _ => unreachable!(),
            }
        }
        _ => {
            println!("Provide day argument between 1 and 24");
        }
    }
}

#[test]
fn all_files_are_ok_open_test() {
    for day in 1..=24 {
        // no panicing here
        drop(read_lines(day));
    }
}
