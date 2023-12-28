pub fn sha9(line: &String) -> String {
    let mut converted = String::new();
  
    for letter in line.chars() {
      if letter.is_alphabetic() {
        converted.push(get_char_code(letter));
      } else {
        converted.push(letter);
      }
    }
  
    converted
  }

  fn get_char_code(letter: char) -> char {
    match letter {
      'A' => 'J',
      'B' => 'K',
      'C' => 'L',
      'D' => 'M',
      'E' => 'N',
      'F' => 'O',
      'G' => 'P',
      'H' => 'Q',
      'I' => 'R',
      'J' => 'S',
      'K' => 'T',
      'L' => 'U',
      'M' => 'V',
      'N' => 'W',
      'O' => 'X',
      'P' => 'Y',
      'Q' => 'Z',
      'R' => 'A',
      'S' => 'B',
      'T' => 'C',
      'U' => 'D',
      'V' => 'E',
      'W' => 'F',
      'X' => 'G',
      'Y' => 'H',
      'Z' => 'I',
      'a' => 'j',
      'b' => 'k',
      'c' => 'l',
      'd' => 'm',
      'e' => 'n',
      'f' => 'o',
      'g' => 'p',
      'h' => 'q',
      'i' => 'r',
      'j' => 's',
      'k' => 't',
      'l' => 'u',
      'm' => 'v',
      'n' => 'w',
      'o' => 'x',
      'p' => 'y',
      'q' => 'z',
      'r' => 'a',
      's' => 'b',
      't' => 'c',
      'u' => 'd',
      'v' => 'e',
      'w' => 'f',
      'x' => 'g',
      'y' => 'h',
      'z' => 'i',
      _ => ' '
    }
  }
  
  #[test]
  fn it_converts() {
    let input = "Hello World!".to_string();
    let converted = "Qnuux Sknhz!".to_string();
    assert!(sha9(&input) == "Qnuux Sknhz!");
    assert!(sha9(&converted) == "Hello World!");
  }