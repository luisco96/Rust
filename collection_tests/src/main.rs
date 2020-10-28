use std::collections::HashMap;

fn main() {
    // first we need to make a function that returns the mean(average) of a vector
    let mut vector = vec![12, 32, 43, 55 ,66 ,56, 54, 65, 67, 33, 23, 11, 43, 54, 33,
                      54,  43, 43, 23, 67];

    let average = get_average(&vector);

    println!("The Vector contains {:?}\n", vector);
    println!("The average of the array is {}", average);

    vector.sort_unstable();
    let median = vector[vector.len()/2];
    println!("The median of the array is {}", median);

    let mode = get_mode(&vector);
    println!("The mode of the vector is {}", mode);

    // pig latin strings

    let message = String::from("Few societies treasured dignity, and feared humiliation,\
                                as did the Japanese, for whom a loss of honor could merit\
                                suicide. This is likely one of the reasons why Japanese s\
                                oldiers in World War II debased their prisoners with such\
                                zeal, seeking to take from them that which was most painf\
                                ul and destructive to lose.");

    let mut chars = message.chars().peekable(); // this returns a peekable iterator that goes through the string
    let mut latin_message = String::new();

    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' |
            'A' | 'E' | 'I' | 'O' | 'U' => {
                latin_message.push(c);
                String::from("-hay")
            }
            'a'..='z' | 'A'..='Z' => {
                format!("-{}ay", c)
            }
            _ => {
                latin_message.push(c);
                continue;
            }
        };

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    chars.next();
                    latin_message.push(c);
                }
                _ => break,
            }
        };
        latin_message += &suffix;
    }

    println!("Original string:\n{}",message);
    println!("Converted string:\n{}",latin_message);
}

fn get_average(vector: &[i32]) -> f64 {
    let mut sum = 0;

    for i in vector.iter() {
        sum += *i;
    }

    sum as f64 / vector.len() as f64
}

fn get_mode(vector: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();

    for i in vector.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_value = 0;
    let mut max_key = 0;
    for (key, value) in &map {
        if *value > max_value {
            max_key = *key;
            max_value = *value;
        }
    }
    max_key
}
