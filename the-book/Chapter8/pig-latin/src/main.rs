// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let text = String::from("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding");

    // separate in words
    let text = text.replace(&['(', ')', ',', '\"', '.', ';', '-', ':', '\'', '“', '”'][..], "");
    let words : Vec<&str> = text.split(" ").collect();

    let mut converted_words = Vec::<String>::new();
    
    for word in words {
        converted_words.push(eng2pig_latin_word(word));
    }

    // reassemble the text
    let text : String = converted_words.join(" ");

    println!("{text}");
}


fn eng2pig_latin_word(word : &str) -> String {
    let first_letter = match word.chars().nth(0) {
        Some(value) => value,
        None => return "".to_string(),
    };

    let converted_word : String = match first_letter {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => format!("{}-hay", word),
        _ => {
            let chars : Vec<&str> = word.split("").collect();
            format!("{}-{}ay", chars[1..].concat(), first_letter)
        }
    };
    converted_word
} 