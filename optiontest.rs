//Option<T> replaces null — every other language lets null sneak in anywhere and blow up at runtime. Rust makes "this might not exist" an explicit, type-safe contract you must handle.
#![allow(unused)] 
pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn test_option_string() -> Option<String>{
    let mut opt1: Option<String> = None;
    opt1 = Some("Reet Solanki".to_string()); // to_string converts pointer to String(i.e extendable)
    return opt1;
}

// struct — all fields exist at once
// enum — only ONE variant is active at a time, An enum says "this value is exactly ONE of these possible things"

pub fn test_option_chartype() -> Option<Charactertype>{
    let mut chartype: Option<Charactertype> = None;
    chartype = Some(Charactertype::Warrior);
    return chartype;
}

pub enum Charactertype {
    Archer, 
    Warrior,
    Mage
}

impl ToString for Charactertype {
    fn to_string(&self) -> String {
        match self {
            Charactertype::Archer => "Archer",
            Charactertype::Mage => "Mage",
            Charactertype::Warrior => "Warrior"
        }.to_string()
    }
}