use std::fs;

pub fn read_somefile(){
    let file_to_read = "./data/file03.txt";

    let read_result = std::fs::read(file_to_read);

    let convert_bytes_to_string = |mut a:String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        return a;
    };

    if read_result.is_ok(){
        println!("Data found is {}", read_result.ok().unwrap().iter().fold(String::from(""), convert_bytes_to_string));
    }
}

pub fn remove_dir(){
    let path = "./data";
    _ = std::fs::remove_dir(path);
}

pub fn create_file(){
    let path = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";
    let text = "Reet Solanki";
    let text2 = "Adi Bhati";
    let text3 = "Kunal Purohit";

    _ = std::fs::write(path, text);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

    // _ = std::fs::remove_file(path2); //Removes file at path
}

pub fn test_create_dir(){
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists(){
        println!("Directory already exists!");
        return;
    }

    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok(){
        println!("Created new data directory");
    }
    else{
        println!("Some problem: {:?}", create_dir_result.err());
    }

}