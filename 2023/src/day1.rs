use crate::utils;

pub(crate) fn day_main() {
    println!("Hello World");
    let file_name: &str = "src/day1_input.txt";
    let mut sum:u32 = 0;
    if let Ok(lines) = utils::read_file(file_name) {
        for line in lines.flatten() {
            let mut first: char = '\0';
            let mut last: char = '\0';
            for c in line.chars() {
                if c.is_digit(10) {
                    if first == '\0' {
                        first = c;
                        continue;
                    }
                    last = c;
                }
            }
            if last == '\0' {
                last = first
            }
            let val:String = format!("{}{}", first, last);

            println!("{}", val);
            sum = sum + val.parse::<u32>().unwrap();

        }
    }
    println!("Day 1 Result: {}", sum);

}