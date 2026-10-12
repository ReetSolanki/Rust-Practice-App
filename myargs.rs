// #![allow(unused)]
// Put input in terminal as  "cargo run -- Cheyenne 89"
use std::env::args;

pub fn process_args() {
    let mut myargs: Vec<String> = args().collect();

    println!("{:?}", myargs);

    if myargs.len() != 3 {
        // Here myargs consumes the data on it so we use it reference than consuming it
        println!("Hey you didn't specify two Arguments"); // One argument is binary automatically
        return;
    }

    let name: String = myargs.get(1).unwrap().into(); //(Just write commands one-by-one you will get it)
    let parsed_year = myargs.get(2).unwrap().parse::<i32>();
    if !parsed_year.is_ok() {
        println!("Please put an integer value");
        return;
    }
    let year_born: i32 = parsed_year.unwrap();

    println!("{name} {year_born}");
    let dog01 = new_dog(name, year_born);
    dog01.get_details();
}

fn new_dog(name: String, year_born: i32) -> Dog {
    return Dog { name, year_born }; // If struct filds matches with input then we don't have to include name:name   
}

struct Dog {
    name: String,
    year_born: i32,
}

impl Dog {
    fn get_details(&self) {
        println!("Dog name {}, and born in {}", self.name, self.year_born);
    }
}
