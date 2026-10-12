use std::collections::{HashMap};

pub fn tes_hashmap_basic(){
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    println!("{}", stock_list.len());

    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AMPL".to_string(), 987.92);
    stock_list.insert("GOOG".to_string(), 873.53);
    
    println!("{:#?}", stock_list); // Using # makes the output clean and more readable with newline and igmentation
    
    stock_list.insert("GOOG".to_string(), 990.32);

    stock_list.entry("META".to_string()).or_insert(342.89); // To avoid accidently overwritting an item
    stock_list.entry("META".to_string()).or_insert(1000.00); 

    stock_list.remove(&("AMPL".to_string()));

    println!("{:#?}", stock_list);

    for (ticker, current_value) in stock_list {
        println!("{} is trading at {}", ticker, current_value);
    }
}