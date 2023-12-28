pub fn sha912(line: &str, encrypt: bool) -> String {
    let mut converted = String::new();
    let mut shift = if encrypt { 9 } else { -9 };
  
    for letter in line.chars() {
      if letter.is_alphabetic() {
        converted.push(shift_char(letter, shift));
        shift = update_shift(shift, encrypt);
      } else {
        converted.push(letter);
      }
    }
  
    converted
  }

  fn shift_char(letter: char, shift: i32) -> char {
    let (start, end) = if letter.is_uppercase() { ('A', 'Z') } else { ('a', 'z') };
    let first_ascii = start as i32;
    let last_ascii = end as i32;
    let alphabet_length = 26;

    let mut shifted_ascii = (letter as i32) + shift;

    while shifted_ascii > last_ascii {
      shifted_ascii -= alphabet_length;
    }
    while shifted_ascii < first_ascii {
      shifted_ascii += alphabet_length;
    }

    std::char::from_u32(shifted_ascii as u32).unwrap_or(letter)
  }

  fn update_shift(shift: i32, encrypt: bool) -> i32 {
    let mut new_shift = shift + if encrypt { -1 } else { 1 };
    if new_shift == 2 || new_shift == -2 {
        new_shift += if encrypt { 9 } else { -9 };
    }
    new_shift
}
  
  #[test]
  fn it_converts() {
    let input = "Hello World".to_string();
    let converted = "Qmsrt Arcvm".to_string();
    assert!(sha912(&input, true) == "Qmsrt Arcvm");
    assert!(sha912(&converted, false) == "Hello World");
  }