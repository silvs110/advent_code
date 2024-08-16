mod day1;
mod utils;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    day1::day_main()
}
