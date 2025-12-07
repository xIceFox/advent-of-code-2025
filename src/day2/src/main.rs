mod tests;

/*fn is_invalid_id(id: u64) -> bool {
    let mut repeating_num: Vec<u8> = vec![];
    let mut read_in_buffer: Vec<u8> = vec![];
    let mut index: usize = 0;

    let mut id_copy = id.clone();
    loop {
        let digit = (id_copy % 10) as u8;
        id_copy /= 10;

        if repeating_num.len() > 0 && digit == repeating_num[index] {
            index += 1;
            read_in_buffer.push(digit);
        } else if read_in_buffer.len() > 0 {
            repeating_num.append(&mut read_in_buffer);
            repeating_num.push(digit);
            read_in_buffer = vec![];
            index = 0;
        } else {
            repeating_num.push(digit);
        }

        if index == repeating_num.len() && id_copy == 0 {
            return true;
        }

        if index == repeating_num.len() && id_copy != 0 {
            repeating_num.append(&mut read_in_buffer);
            read_in_buffer = vec![];
            index = 0
        }

        if id_copy == 0 {
           return false;
        }
    }
}*/

fn matches_pattern(pattern : &str, string: &str) -> bool{
    let pattern_char = pattern.chars().collect::<Vec<char>>();
    let string_chars = string.chars().collect::<Vec<char>>();

    for i in  0..string_chars.len(){
        if string_chars[i] != pattern_char[i % pattern_char.len()]  {
            return false;
        }
    }

    true
}

fn is_invalid_id_part1(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (left, right) = s.split_at(len/2);

    left == right
}

fn is_invalid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for pattern_len in 1..=len/2{
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let (pattern, _) = s.split_at(pattern_len);

        if matches_pattern(pattern, &s){
            return true
        }
    }

    false
}

fn main() {
    const PATH: &str = "src/day2/input.txt";
    let lines = core::read_lines(PATH).unwrap();

    let mut result_part1: u64 = 0;
    let mut result_part2: u64 = 0;
    for line in lines {
        if line.is_err(){
            continue;
        }

        let value = line.unwrap();
        let parts = value.split(",");
        for part in parts {
            let range_parts = part.split("-").collect::<Vec<&str>>();

            if range_parts.len() != 2 {
                continue;
            }

            let start_result = range_parts[0].parse::<u64>();
            let finish_result = range_parts[1].parse::<u64>();

            if start_result.is_err() || finish_result.is_err() {
                continue;
            }

            let start = start_result.unwrap();
            let finish = finish_result.unwrap();

            for num in start..finish+1 {
                if is_invalid_id_part1(num) {
                    result_part1+= num;
                }

                if is_invalid_id_part2(num) {
                    result_part2+= num;
                }
            }
        }
    }

    println!("{}", result_part1);
    println!("{}", result_part2);
}