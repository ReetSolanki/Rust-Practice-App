//use std::{cell::Cell, default};

#[derive(Debug)]  // Allows us to print the entire struct and its info, without calling individually
#[allow(dead_code)]
enum VehicleColor {
    Silver, 
    White, 
    Red,
    Black,
    Green
}

// Tuple based struct instead of calling like VehicleTuple.name, we use VehicleTuple.0
#[derive(Debug)]
#[allow(dead_code)] 
struct VehicleTuple(String, String, u16);

#[allow(dead_code)]
#[derive(Debug)] // Allows us to print the entire struct and its info, without calling individually
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

// Methods are used if you want to restrict certain colors based on rules, for eg if I use v1.color = VehicleColor::White;, it is done directly but with 'methods' I can put restriction like vintage cars can't be painted white and throw an error
impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) { // Instance level method, we use period
        self.color = new_color;
    }

    fn create_vehicle() -> Vehicle{ // Static method, we use :: to call it
        let new_vehicle = Vehicle{
            manufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 1995,
            color: VehicleColor::Silver
        };
        return new_vehicle; 
    }
}

struct Person {
    frist_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
    meters_walked: u32,
}

impl Person {
    fn walk(&mut self, meters: u32) {
        self.meters_walked+=meters;
    }
}

pub fn create_vehicletuple(){
    println!("{:?}", new_vehicletuple());
    let myvehicletuple: VehicleTuple = new_vehicletuple();
    println!("{0} and {1}", myvehicletuple.0, myvehicletuple.1);
}

fn new_vehicletuple () -> VehicleTuple{
    return VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2015);
}

#[allow(dead_code)]
fn new_vehicle () -> Vehicle{
    let mut v1: Vehicle = Vehicle{
        manufacturer: "Porche".to_string(),
        color: VehicleColor::Silver,
        model: "911".to_string(),
        year: 1991
    };
    v1.paint(VehicleColor::Black);   // Instance level method
    return v1;
}

pub fn create_vehicle () {
    //let myvehicle: Vehicle = new_vehicle();
    let mut myvehicle = Vehicle::create_vehicle();   // Static method
    myvehicle.paint(VehicleColor::Black);
    println!("{:?}", myvehicle);
}

fn new_person () -> Person {
    let p1 = Person{ //Just the firstname can be made mutable by using Cell (use mut to make all fields mutable)
        frist_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
        birth_month: 6,
        birth_year: 1986,
        visited_europe: false,
        meters_walked: 0
    };
    //p1.frist_name.set("Shannon");
    return p1;
}

pub fn test_create_person () {
    let mut myperson: Person = new_person();
    myperson.walk(10);
    myperson.walk(20);
    println!("{0} {1} {2}, {3}, {4}, {5}", myperson.frist_name, myperson.last_name, myperson.birth_month, myperson.birth_year, myperson.visited_europe, myperson.meters_walked);
}