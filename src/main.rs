use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    match run() {
        Ok(answer) => {
            println!("CAPTCHA value is {}", answer);
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
        }
    }
}

fn run() -> Result<usize, Box<Error>> {
    let input = File::open("puzzle-input.txt")?;
    let mut reader = BufReader::new(input);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let digits: Vec<usize> = contents
        .chars()
        .filter_map(|digit| match digit {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            _ => None,
        })
        .collect();

    let answer = captcha(&digits);
    Ok(answer)
}

fn captcha(digits: &[usize]) -> usize {
    let next_digit = digits.iter().cycle().skip(1);

    digits
        .iter()
        .zip(next_digit)
        .filter_map(|(digit, next)| if digit == next { Some(digit) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_one_two_two_gives_three() {
        assert_eq!(captcha(&[1, 1, 2, 2]), 3);
    }

    #[test]
    fn four_ones_gives_four() {
        assert_eq!(captcha(&[1, 1, 1, 1]), 4);
    }

    #[test]
    fn one_two_three_four_gives_zero() {
        assert_eq!(captcha(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn sequence_starting_and_ending_with_nine() {
        assert_eq!(captcha(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }
}
