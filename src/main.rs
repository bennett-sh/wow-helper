use inquire::{validator::Validation, Confirm, CustomType, Text};
use std::error;

const WORDS_RAW: &str = include_str!("../wordlist.txt");
const MIN_LENGTH: usize = 3;

fn broad_field(characters: &Vec<char>) -> Result<Vec<String>, Box<dyn error::Error>> {
  let words = WORDS_RAW
    .to_uppercase()
    .split('\n')
    .map(|word| word.trim())
    .filter(|word| {
      if word.len() < MIN_LENGTH {
        return false;
      }
      let mut chars_remaining = characters.clone(); // Make a clone to track used characters per word
      for ch in word.chars() {
        match chars_remaining.iter().position(|&x| x == ch) {
          Some(index) => {
            chars_remaining.remove(index);
          }
          None => return false,
        }
      }
      true
    })
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  Ok(words)
}

fn specific_field(characters: &Vec<char>) -> Result<Vec<String>, Box<dyn error::Error>> {
  let word_len = CustomType::<usize>::new("How long is the word?").prompt()?;
  let mut known_characters: Vec<Option<char>> = Vec::new();

  if Confirm::new("Do you already know any characters?").prompt()? {
    let raw_validator = move |input: &str| {
      if input.len() > word_len {
        return Ok(Validation::Invalid(
          format!(
            "The word only has {} characters. You supplied {}.",
            word_len,
            input.len()
          )
          .into(),
        ));
      }
      if input.chars().all(|x| x.is_alphabetic() || x == '-') {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(
          "You can only use alphabetical characters & dashes.".into(),
        ))
      }
    };
    let raw = Text::new("Please enter the characters you know (use dashes for unknown characters)")
      .with_validator(raw_validator)
      .prompt()?;

    for character in raw.to_uppercase().chars() {
      known_characters.push(if character == '-' {
        None
      } else {
        Some(character)
      })
    }
  }

  let words = WORDS_RAW
    .to_uppercase()
    .split('\n')
    .map(|word| word.trim())
    .filter(|&word| {
      if word.len() != word_len {
        return false;
      }

      let mut chars_remaining = characters.clone();
      for ch in word.chars() {
        match chars_remaining.iter().position(|&x| x == ch) {
          Some(index) => {
            chars_remaining.remove(index);
          }
          None => return false,
        }
      }

      for i in 0..known_characters.len() {
        if i >= word_len {
          continue;
        }
        if let Some(ch) = known_characters[i] {
          if let Some(other_ch) = word.chars().nth(i) {
            if ch != other_ch {
              return false;
            }
          }
        }
      }
      true
    })
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  Ok(words)
}

fn do_turn(characters: &Vec<char>) -> Result<(), Box<dyn error::Error>> {
  let mut words = if Confirm::new("Do you want to find out a specific field?").prompt()? {
    specific_field(characters)?
  } else {
    broad_field(characters)?
  };
  words.sort_by(|a, b| {
    let length_comparison = a.len().cmp(&b.len());
    if length_comparison == std::cmp::Ordering::Equal {
      a.cmp(b)
    } else {
      length_comparison
    }
  });
  words.dedup();

  print!("Possible words:");
  if words.len() == 0 {
    println!(" none.");
  } else {
    println!("");
  }
  for word in words {
    println!("{word}");
  }

  Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
  let characters_validator = |input: &str| {
    if input.chars().all(|x| x.is_alphabetic()) {
      Ok(Validation::Valid)
    } else {
      Ok(Validation::Invalid(
        "You can only use alphabetical characters.".into(),
      ))
    }
  };
  let characters_str = Text::new("Which characters can you use?")
    .with_validator(characters_validator)
    .prompt()?
    .to_uppercase();
  let characters = characters_str.chars().collect::<Vec<char>>();

  loop {
    do_turn(&characters)?;
  }
}
