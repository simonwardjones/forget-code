use std::collections::HashMap;

fn main() {
    println!("Collection examples");
    println!();
    vector_examples();
    string_examples();
    hash_examples();
    println!();
    collection_questions::run_questions();
}

fn vector_examples() {
    println!("Vector example");
    let mut v: Vec<i32> = Vec::new(); // initialise empty vector
    let v2 = vec![1, 2, 3]; // initialise vector with values using vec! macro
    v.push(1); // append
    v.push(20); // append
    v.extend(&v2); // extend
    for i in &mut v {
        // loop
        *i += 10;
    }

    let thirty = &v[1]; // gets value at index 1
    let not_long_enough = v.get(100); // gets value at index 100 or None
    println!("thirty is {thirty}");
    match not_long_enough {
        Some(value) => println!("not_long_enough is {value}"),
        None => println!("not_long_enough is None"),
    }
    println!("v is {v:?}");
    println!();
}

#[allow(unused_variables)]
fn string_examples() {
    println!("String examples");
    let empty_immutable_string = String::new(); // initialise empty String
    let mut s = String::from("simon"); // initialise String from &str
    let mut si = "simon".to_string(); // also initialise String from &str
                                      // string modifying
    if s == si {
        println!("Before modifying s and si are equal");
    }
    s.push_str(" says hello"); // push_str appends a literal to a String
    si.push('!'); // push appends a single character to a String
    println!("s is \"{s}\"");
    println!("si is \"{si}\"");
    let here = String::from("here");
    let we = String::from("we");
    let go = String::from("go");
    let here_we_go = String::new() + &here + " " + &we + " " + &go + "!";
    // this is a little unwieldy so we can use format!
    let here_we_go_again = format!("{here} {we} {go} again!");
    println!("{here_we_go}");
    println!("{here_we_go_again}");

    // emojis
    let contains_emoji = "Yes ✅";
    println!("contains_emoji is \"{contains_emoji}\"");
    let len = contains_emoji.len();
    let char_len = contains_emoji.chars().collect::<Vec<char>>().len();
    for (i, character) in contains_emoji.chars().enumerate() {
        println!("{i}, char:'{character}'");
        let t = character as u32;
        println!("{t}")
    }
    for (i, byte) in contains_emoji.bytes().enumerate() {
        println!("{i}, byte:{byte}");
    }
    println!("{}", contains_emoji);
    println!("contains_emoji.len() is {len}");
    println!("contains_emoji.len() is {char_len}");
    println!();
}

fn hash_examples() {
    println!("Hash examples");
    let mut basket: HashMap<String, i32> = HashMap::new();
    let apple: String = "apple".to_owned();
    let banana: String = String::from("banana");
    let apple_clone = apple.clone();
    basket.insert(apple, 3);
    basket.insert(banana, 2);
    // For types implementing Copy trait, like i32, the values are copied
    // For types not implementing Copy trait, like String, the values are moved
    // So we cannot reference apple now!
    println!("basket={basket:?}");

    // Access values
    let _a = basket.get(&apple_clone).expect("should be there");
    let _b = basket["banana"];

    // Overwriting a value
    basket.insert(apple_clone, 10);
    println!("basket={basket:?}");
    // adding if not there
    basket.entry(String::from("orange")).or_insert(0);
    basket.entry(String::from("banana")).or_insert(0);
    println!("basket={basket:?}");
    // updating a value based on old value
    basket.entry(String::from("orange")).and_modify(|v| *v += 1);
    let bananas_entry = basket.entry(String::from("banana")).or_insert(0);
    *bananas_entry += 100;
    println!("basket={basket:?}");

    // Iterating over a hash map happens in an arbitrary order.
    for (key, value) in &basket {
        println!("{key}={value}");
    }
}

pub mod collection_questions {
    use std::collections::{HashMap, VecDeque};

    pub fn question_1(vector: &Vec<i32>) -> (f64, i32) {
        // Given a list of integers, use a vector and return the median
        // (when sorted, the value in the middle position) and mode
        // (the value that occurs most often; a hash map will be helpful here) of the list.

        // median
        let mut vector = vector.to_vec();
        vector.sort();
        let len = vector.len();
        let median = match len % 2 == 0 {
            true => (vector[len / 2] + vector[(len / 2) - 1]) as f64 / 2 as f64,
            false => vector[(len - 1) / 2] as f64,
        };

        // mode
        let mut value_counts: HashMap<i32, i32> = HashMap::new();
        for value in &vector {
            let count = value_counts.entry(*value).or_insert(0);
            *count += 1;
        }
        println!("value_counts is {value_counts:?}");
        let mut mode_pair = (0, 0);
        for (value, count) in value_counts {
            if count > mode_pair.1 {
                mode_pair = (value, count);
            }
        }
        // another mode implementation (slicker)
        let mut counts = HashMap::new();
        let mode = vector
            .iter()
            .max_by_key(|a| {
                let count = counts.entry(*a).or_insert(0);
                *count += 1;
                *count
            })
            .unwrap();
        // return (median, mode_pair.0);
        return (median.into(), *mode);
    }

    fn _get_words(string: &str) -> Vec<String> {
        let mut passing_word = true;
        let mut words: Vec<String> = Vec::new();
        let mut word: Vec<char> = Vec::new();
        for (i, letter) in string.chars().enumerate() {
            if i == 0 {
                passing_word = letter.is_alphabetic();
            }
            if letter.is_alphabetic() && passing_word {
                word.push(letter);
            } else if letter.is_alphabetic() && !passing_word {
                words.push(word.iter().collect());
                word = vec![letter];
                passing_word = true;
            } else if !letter.is_alphabetic() && passing_word {
                words.push(word.iter().collect()); //push word
                word = vec![letter];
                passing_word = false;
            } else if !letter.is_alphabetic() && !passing_word {
                word.push(letter);
            }
        }
        if !word.is_empty() {
            words.push(word.iter().collect());
        }
        words
    }

    fn _convert_word_to_pig(string: &str) -> String {
        let mut chars = string.chars().collect::<VecDeque<char>>();
        let first = chars.pop_front().unwrap();
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        if vowels.contains(&first) {
            chars.extend("hay".chars());
        } else {
            chars.extend((first.to_string() + "ay").chars());
        }
        chars.iter().collect::<String>()
    }

    pub fn question_2(string: &str) -> String {
        // Convert strings to pig latin. The first consonant of each word is moved
        // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
        // Words that start with a vowel have “hay” added to the end instead
        // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
        let out = _get_words(string)
            .iter()
            .map(|x| {
                if x.chars().next().unwrap().is_alphabetic() {
                    _convert_word_to_pig(x)
                } else {
                    x.to_string()
                }
            })
            .collect::<String>();
        out
    }
    pub fn run_questions() {
        let example = vec![1, 2, 3, 4, 5, 6, 6, 6, 7, 8];
        let (median, mode) = question_1(&example);
        println!("median, mode is is {median}, {mode}");

        let example = "!!!";
        let pig_latin = question_2(&example);
        println!("pig_latin: {pig_latin}");
    }
}
