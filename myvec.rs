use std::vec;

pub fn tes_vec_int () {
    let mut my_int: Vec<u32> = Vec::new();
    my_int.push(30);
    my_int.push(84);
    my_int.push(84);
    my_int.push(84);
    my_int.push(84);
    my_int.push(84);

    println!("Size of vector is {:?}", my_int.len());
    println!("Capacity of vector is {:?}", my_int.capacity());
    println!("First item in vector is {}", my_int[0]);
    println!("{:?}", my_int);
    println!("{:?}", &(&my_int).as_slice()[0..=5]);
    println!("{:?}", my_int.get(1));
}

pub fn test_vec_string (){
    let first_names = vec!["Adi", "Reet", "KUnal", "Arpit"];

    for first_name in first_names.clone(){
        println!("Processing for {}", first_name);
    }

    println!("{:?}", first_names); //iterating in for loop destroys/consumes the info in first_names, so we use as_slice/clone to get a copy to iterate over
}

#[derive(Debug)]
#[allow(dead_code)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_car(){
    let mut car_list: Vec<Car> = vec![];

    let mut car_lot2: Vec<Car> = vec![];

    for _ in 1..=50u32 {
        car_list.push(Car{manufacturer: "Porche".to_string(), model: "Panamera".to_string()});
    }

    for _ in 1..=50u8 {
        car_lot2.push(Car{manufacturer: "Hyundai".to_string(), model: "Sonata".to_string()});
    }

    car_list.append(&mut car_lot2); //Contents of car_lot2 are pushed & transffered into car_list
    car_list.insert(0, Car{manufacturer: "Lambo".to_string(), model: "Avantador".to_string()});//Inserts an element in the starting and pushes rest elements further 
    car_list.remove(0); // Pass the index you want to remove

    let keep = |e: &Car| {if e.manufacturer == "Porche" {return true;} else {return false;}};
    car_list.retain(keep);  // Scans the vec and retains only those elements those have manufacturer Porche

    car_list.reserve(5000); //Reserve additional space for car_list

    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_list.capacity());
    println!("{:?}", car_lot2.len());

    println!("{:?}", car_list.get(0).unwrap());

    // Testing for btop
    let mut input = "".to_string();
    std::io::stdin().read_line(&mut input).expect("Something bad happened!");
}