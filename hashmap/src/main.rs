use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // we can do a little magic trick to create a hash map from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);
    println!("Score is {}", score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating Hash maps
    // Replacing old value with a new one
    scores.insert(String::from("Orange"), 15);
    scores.insert(String::from("Orange"), 30);
    println!("{:?}", scores);

    // Inserting if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    // Updating a value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
