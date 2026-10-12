#![allow(unused)] 
use std::collections::HashSet;

pub fn test_hashset_type() {
    let mut planet_list: HashSet<&str> = HashSet::from(["Mercury", "Venus", "Earth"]);
    let planet_list_more: HashSet<&str> = HashSet::from(["Jupyter", "Earth", "Mars "]);

    let planet_diff: HashSet<&str> = planet_list // .copied and collect are used to convert &&str → &str.
        .difference(&planet_list_more)
        .copied()
        .collect(); // Only removes common planet from planet_list

    let planet_symmdiff: HashSet<&str> = planet_list
        .symmetric_difference(&planet_list_more)
        .copied()
        .collect(); // Removes common planet and takes union

    // for planet in planet_list{
    //     println!("Thanks for adding {}", planet);
    // }
    // for planet in planet_diff{
    //     println!("Thanks for adding {}", planet);
    // }

    planet_list.insert("Saturn");
    planet_list.insert("Uranus");
    planet_list.insert("Neptune");
    planet_list.insert("Pluto");

    for planet in planet_list{
        println!("Thanks for adding {}", planet);
    }
}
