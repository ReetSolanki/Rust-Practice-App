pub fn test_match_int() {
    let myage: u16 = 2;

    match myage {
        35 | 36 => {
            println!("Your age is 35 or 36");
        }
        156.. => {
            //156 upto max value of u16
            println!("Your age is over 156");
        }
        2..=34 => println!("Your age is between 2 and 34"),
        _ => {
            // Default arm path if nothing matches
            println!("Your age is not something else");
        }
    }
}

pub fn test_match_string() {
    let car_manufacturer = "Porche";

    match car_manufacturer {
        "Hyundai" => {
            println!("Hyundai it is!")
        }
        _ => {
            println!("No match")
        }
    }
}

pub fn test_match_array() {
    let prices: [i32; 3] = [50000, 90000, 12000];

    match prices[0..=2] {
        // trying to match 1st three elements
        [50000, 90000] => println!("You have expensive cars"),
        [50000, 90000, ..] => println!("You have a variety of cars"),
        _ => println!("You don't have expensive cars"),
    };
}
