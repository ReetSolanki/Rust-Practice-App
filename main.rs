//use crate::mystructs::{test_create_person, create_vehicle, create_vehicletuple};

pub mod closures;
pub mod helpers;
pub mod matchtest;
pub mod myargs;
pub mod mydatetime;
pub mod myfs;
pub mod myhashmap;
pub mod myhashset;
pub mod myiters;
pub mod mympsc;
pub mod mymutex;
pub mod myscopedthreads;
pub mod mystructs;
pub mod mythreads;
pub mod myvec;
pub mod optiontest;
pub mod test_traits;

fn main() {
    println!("Hello from Rust, ARK");
    // println!("let's see if this works");

    // //test_func();
    // //helpers::database

    // let myresult: String = helpers::namehelpers::get_full_name("Shane", "Jones");
    // let num: u128 = helpers::database::get_age_plus_5(23);

    // println!("Hello from {0}", myresult);
    // println!("The new age is {0}", num);
    // println!("{0} {1} {0}", "hello", "world"); // → "hello world hello"

    //test_if();
    //test_for();
    //test_closures();
    //matchtest::test_match_int();
    //matchtest::test_match_string();
    //matchtest::test_match_array();

    // let result: Option<u8> = optiontest::test_option_type();
    // println!("{}", result.unwrap());
    // let stresult = optiontest::test_option_string();
    // println!("{}", stresult.unwrap());
    // let charresult: Option<Charactertype> = optiontest::test_option_chartype();
    // if charresult.is_some() {
    //     println!("User has selected character type");
    //     println!("Character selected is: {}", charresult.unwrap().to_string());  // Rust doesn't know how to take this custum enum and make it into a string, so we make a function in optiontest.rs
    // }
    // else {
    //     println!("Character type is null");
    // }

    //test_create_person();
    // create_vehicle();
    // create_vehicletuple();

    //test_traits::create_person();

    //myvec::tes_vec_int();
    // myvec::test_vec_string();
    //myvec::test_vec_car();

    //myhashmap::tes_hashmap_basic();
    //myhashset::test_hashset_type();

    // myiters::test_rust_iterators();

    //mydatetime::test_stdtime();
    // mydatetime::tes_chrono();

    // mythreads::test_threads();
    //mythreads::spawn_thread();

    // myscopedthreads::test_thread_variable();

    // mymutex::test_mutex();

    // mympsc::test_channels();

    // myfs::test_create_dir();
    // myfs::create_file();
    // myfs::remove_dir();
    // myfs::read_somefile();

    // println!("Generate uuid is: {}", uuid::Uuid::new_v4().to_string());

    myargs::process_args();
}

#[allow(dead_code)]
fn test_for() {
    let ages: [i32; 5] = [12, 15, 17, 19, 20];
    let age_to_drive = 16;

    for value in ages {
        if value >= age_to_drive {
            println!("You are old enough to drive, you are {}", value);
        } else {
            println!("Next time kiddo, you are just {}", value);
        }
    }
}

#[allow(dead_code)]
fn test_if() {
    let age_to_drive: u8 = 16u8;

    println!("Enter person age:");
    //let myinput: &mut String = &mut String::from("");
    let mut myinput = String::new();
    std::io::stdin().read_line(&mut myinput).unwrap();

    let age: u8 = myinput.trim().parse::<u8>().unwrap(); //.trim() to strip the newline character that read_line appends
    if age > age_to_drive {
        println!("Old enough to drive");
    } else if age == 16 {
        println!("Almost old enough")
    } else {
        println!("Not old enough kiddo");
    }

    let drivers_license: bool = age >= 16;
    println!("{}", drivers_license);
}

#[allow(dead_code)] //To ignore the function unsed warning
fn test_func() {
    let x: f32 = 5.0;
    let y: u8 = (x as u8) + 9;
    println!("{}", y);

    let iamold: bool = true;

    let myst: &str = "Trevor";

    println!("{:?}", myst);
    println!("{}", iamold);

    let name: (&str, &str, i32) = ("Reet ", "Solanki", 40); //Tuple
    println!("{:?}", name); //Using {:?} gives no eror

    let ages: [i32; 3] = [40, 45, 50];
    println!("{:?}", ages[0]);
}
