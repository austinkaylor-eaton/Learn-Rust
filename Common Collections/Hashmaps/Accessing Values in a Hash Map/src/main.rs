use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let team_name = String::from("Blue");
    
    scores.insert(String::from(&team_name), 10);
    scores.insert(String::from("Yellow"), 50);

    //let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{} team's score is {}", &team_name, score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
