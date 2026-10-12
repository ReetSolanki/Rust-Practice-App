extern crate chrono; // External dependency
use std::time::{Duration, Instant};

use chrono::NaiveDate;

pub fn test_stdtime(){// Time is stored in a common unit so it can be sub directly, negative time doesen't make sense 
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());
    
    let dur2 = Duration::from_millis(512500);

    let dur3 = dur1.checked_sub(dur2); // Return none or a +ve sbutracted time
    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();

    std::thread::sleep(Duration::from_millis(500));
    println!("Time elapsed in sleep: {}", now.elapsed().as_micros());
}

pub fn tes_chrono() { // Using Chrono crate
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now.format("%Z %H %Y %b %d"));
    
    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Z %H %Y %b %d"));

    let date1 = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Sun);
    let unwrapped_date = date1.unwrap();
    println!("{}", unwrapped_date.format("Day of the year is: %j"));

    unwrapped_date.iter_days().take(4).for_each(|d: NaiveDate| println!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2024, 366);
    println!("{}", date2.unwrap().format("%A %B %d"));

    let birthday = NaiveDate::parse_from_str("2024|||09||07", "%Y|||%m||%d"); // ||| is the format we used
    println!("{:#?}", birthday.ok().unwrap());
}