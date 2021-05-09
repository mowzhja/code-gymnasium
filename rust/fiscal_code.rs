/// Source: https://edabit.com/challenge/Pa2rHJ6KeRBTF28Pg

use std::error::Error;

#[derive(Debug)]
enum Gender {
    M,
    F,
}

#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    gender: Gender,
    dob: String, // D/M/YYYY
}

impl Person {
    fn new(name: String, surname: String, gender: Gender, dob: String) -> Self {
        Person {
            name,
            surname,
            gender,
            dob
        }
    }

    /// Generates the Codice Fiscale given all the data.
    ///
    /// Note: the CF is an uppercase, 11-char-long String.
    fn get_CF(&self) -> String {
        let mut CF = String::new();

        extract_from_surname(&self.surname, &mut CF);
        extract_from_name(&self.name, &mut CF);
        extract_from_gender_date(&self.gender, &self.dob, &mut CF);

        assert!(CF.len() == 11);

        CF.to_uppercase()
    }
}

fn count_consonants(target: &str) -> u32 {
    let vowels = Vec::from(['a', 'e', 'i', 'o', 'u']);
    let l_target = target.to_string().to_lowercase();

    l_target
        .chars()
        .filter(|ch| ch.is_alphabetic() && !vowels.contains(ch))
        .count() as u32
}

/// Extract useful CF chars from surname.
fn extract_from_surname(surname: &str, CF: &mut String) {
    let vowels = Vec::from(['a', 'e', 'i', 'o', 'u']);
    let cons_count = count_consonants(surname);
    let l_surname = surname.to_string().to_lowercase();

    if cons_count >= 3 {
        // use first three consonants
        for ch in l_surname.chars() {
            if !vowels.contains(&ch) && CF.len() < 3 {
                CF.push(ch);
            }
        }
    } else if cons_count < 3 && surname.len() >= 3 {
        // use consonants, then add the vowel/s in the order they appear
        let mut ordered_vows = Vec::new();

        for ch in l_surname.chars() {
            if !vowels.contains(&ch) {
                CF.push(ch);
            } else {
                // ordered vowels
                ordered_vows.push(ch);
            }
        }

        // how many letters to get to correct length (3)?
        let missing = (3 - CF.len()) as usize;
        for i in 0..missing {
            CF.push(ordered_vows[i]);
        }
    } else if surname.len() < 3 {
        // use the surname + 'x'
        CF.push_str(&l_surname);
        CF.push('x');
    }
}

/// Extract useful CF chars from name.
fn extract_from_name(name: &str, CF: &mut String) {
    let vowels = Vec::from(['a', 'e', 'i', 'o', 'u']);
    let cons_count = count_consonants(name);
    let l_name = name.to_string().to_lowercase();

    if cons_count == 3 {
        // use first three consonants
        for ch in l_name.chars() {
            if !vowels.contains(&ch) && CF.len() < 6 {
                CF.push(ch);
            }
        }
    } else if cons_count > 3 {
        // use first, third and fourth consonants
        let mut ordered_cons: Vec<char> = Vec::new();

        for ch in l_name.chars() {
            if !vowels.contains(&ch) {
                ordered_cons.push(ch);
            }
        }

        CF.push(ordered_cons[0]); // first
        CF.push(ordered_cons[2]); // third
        CF.push(ordered_cons[3]); // fourth
    } else if cons_count < 3 && name.len() >= 3 {
        // use consonants, then add the vowel/s in the order they appear
        let mut ordered_vows = Vec::new();

        for ch in l_name.chars() {
            if !vowels.contains(&ch) {
                CF.push(ch);
            } else {
                // ordered vowels
                ordered_vows.push(ch);
            }
        }

        // how many letters to get to correct length (3)?
        let missing = (6 - CF.len()) as usize;
        for i in 0..missing {
            CF.push(ordered_vows[i]);
        }
    } else if name.len() < 3 {
        // use the name + 'x'
        CF.push_str(&l_name);
        CF.push('x');
    }
}

/// Extract CF chars from gender and date.
fn extract_from_gender_date(gender: &Gender, dob: &str, CF: &mut String) {
    // last two digits of year
    let mut last_two: String = dob.chars().rev().take(2).collect(); // reverse order
    last_two = last_two.chars().rev().collect();

    CF.push_str(&last_two);

    let sp = dob.split('/').collect::<Vec<_>>();
    CF.push(month_letter(sp[1]));

    match gender {
        Gender::M => {
            let day = sp[0].parse::<u32>().unwrap();
            if day < 10 {
                CF.push('0');
                CF.push_str(sp[0]);
            } else {
                CF.push_str(sp[0]);
            }
        }
        Gender::F => {
            let mut day = sp[0].parse::<u32>().unwrap();
            day += 40;

            CF.push_str(&day.to_string());
        }
    }
}

/// Utility to map months in integer form to months in char form.
fn month_letter(month_int: &str) -> char {
    match month_int {
        "1" => {
            'A'
        }
        "2" => {
            'B'
        }
        "3" => {
            'C'
        }
        "4" => {
            'D'
        }
        "5" => {
            'E'
        }
        "6" => {
            'H'
        }
        "7" => {
            'L'
        }
        "8" => {
            'M'
        }
        "9" => {
            'P'
        }
        "10" => {
            'R'
        }
        "11" => {
            'S'
        }
        "12" => {
            'T'
        }
        _ => {
            'Z'
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matt = Person::new("Matt".to_string(), "Edabit".to_string(), Gender::M, "1/1/1900".to_string());

    assert!(matt.get_CF() == "DBTMTT00A01");
    println!("Test #1 passed");

    let helen = Person::new("Helen".to_string(), "Yu".to_string(), Gender::F, "1/12/1950".to_string());

    assert!(helen.get_CF() == "YUXHLN50T41");
    println!("Test #2 passed");

    let mickey = Person::new("Mickey".to_string(), "Mouse".to_string(), Gender::M, "16/1/1928".to_string());

    assert!(mickey.get_CF() == "MSOMKY28A16");
    println!("Test #3 passed");

    Ok(())
}
