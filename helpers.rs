pub mod namehelpers{
    pub fn get_full_name(first: &str, last: &str) -> String{
        let full_name: String = format!("{0} {1}", first, last); //0,1 to show which order they go in, and space also is being included
        return full_name;
    } 
}

pub mod database{
    pub fn get_age_plus_5(age: u128) -> u128{
        return age + 5;
    }
}