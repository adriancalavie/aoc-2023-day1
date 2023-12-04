use std::{fs, ops::Add};

/// The `const DIGITS: &[&str] = &[...]` line is defining a constant array of string slices. Each string slice represents a
/// digit from one to nine. This array is used in various functions to match and convert alphabetic digits to their
/// corresponding numeric values.
const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// The function `find_first_alpha_digit` takes a string as input and returns the first occurrence of a substring that
/// consists of either alphabetic characters or digits, along with its index in the original string.
///
/// Arguments:
///
/// * `str`: The `str` parameter is a reference to a string (`&str`). It represents the input string in which we want to
/// find the first occurrence of a substring that consists of either an alphabetic character or a digit.
///
/// Returns:
///
/// The function `find_first_alpha_digit` returns an `Option` containing a tuple `(String, usize)`.
fn find_first_alpha_digit(str: &str) -> Option<(String, usize)> {
    DIGITS
        .iter()
        .filter_map(|&substring| {
            str.find(substring)
                .map(|index| (substring.to_string(), index))
        })
        .min_by_key(|&(_, index)| index)
}

/// The function `find_last_alpha_digit` finds the last occurrence of a substring consisting of only alphabetic characters
/// and digits in a given string and returns the substring along with its index.
///
/// Arguments:
///
/// * `str`: The `str` parameter is a reference to a string slice (`&str`). It represents the input string that we want to
/// search for the last occurrence of a substring that consists of both alphabetic characters and digits.
///
/// Returns:
///
/// The function `find_last_alpha_digit` returns an `Option<(String, usize)>`.
fn find_last_alpha_digit(str: &str) -> Option<(String, usize)> {
    let mut last_index = None;
    let mut last_substring = None;

    for &substring in DIGITS {
        if let Some(index) = str.rfind(substring) {
            if last_index.map_or(true, |last| index > last) {
                last_index = Some(index);
                last_substring = Some((substring.to_string(), index));
            }
        }
    }

    last_substring
}

/// The function `alpha_to_numeric` takes a string representing an alphabetic digit and returns the corresponding numeric
/// digit, or `None` if the input is not a valid alphabetic digit.
///
/// Arguments:
///
/// * `alpha_digit`: The `alpha_digit` parameter is a reference to a `String` that represents an alphabetic digit.
///
/// Returns:
///
/// The function `alpha_to_numeric` returns an `Option<String>`.
fn alpha_to_numeric(alpha_digit: &String) -> Option<String> {
    DIGITS
        .iter()
        .position(|&digit| digit.eq(alpha_digit))
        .map(|result| (result as u8 + 1).to_string())
}

/// The function `find_numeric_digit` takes an iterator of characters and returns the first numeric digit along with its
/// index, if found.
/// 
/// Arguments:
/// 
/// * `iter`: The `iter` parameter is an iterator that yields characters (`char`).
/// 
/// Returns:
/// 
/// The function `find_numeric_digit` returns an `Option` containing a tuple `(String, usize)`.
fn find_numeric_digit<I>(iter: I) -> Option<(String, usize)>
where
    I: IntoIterator<Item = char>,
{
    iter.into_iter()
        .enumerate()
        .find(|(_, c)| c.is_numeric())
        .map(|(idx, char)| (char.to_string(), idx))
}

/// The function `rev_find_numeric_digit` takes an iterator of characters, collects them into a vector, and then finds the
/// last numeric digit along with its index.
///
/// Arguments:
///
/// * `iter`: The `iter` parameter is an iterator that yields characters (`char`).
///
/// Returns:
///
/// The function `rev_find_numeric_digit` returns an `Option` containing a tuple `(String, usize)`.
fn rev_find_numeric_digit<I>(iter: I) -> Option<(String, usize)>
where
    I: IntoIterator<Item = char>,
    I::IntoIter: DoubleEndedIterator<Item = char>,
{
    iter.into_iter()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
        .rev()
        .find(|(_, c)| c.is_numeric())
        .map(|(idx, char)| (char.to_string(), idx))
}

/// The function `find_first_numeric_digit` takes a string as input and returns the first numeric digit found along with its
/// index, if any.
///
/// Arguments:
///
/// * `str`: The `str` parameter is a reference to a string slice (`&str`). It represents the input string that we want to
/// search for the first numeric digit.
fn find_first_numeric_digit(str: &str) -> Option<(String, usize)> {
    find_numeric_digit(str.chars())
}

/// The function `find_last_numeric_digit` takes a string as input and returns the last numeric digit found in the string
/// along with its index, if any.
///
/// Arguments:
///
/// * `str`: The `str` parameter is a reference to a string slice (`&str`). It represents the input string that we want to
/// search for the last numeric digit.
fn find_last_numeric_digit(str: &str) -> Option<(String, usize)> {
    rev_find_numeric_digit(str.chars())
}

/// The function `get_first_digit` returns the first numeric or alphabetic digit found in a given string, prioritizing
/// numeric digits over alphabetic digits.
///
/// Arguments:
///
/// * `line`: The `line` parameter is a string that represents a line of text.
///
/// Returns:
///
/// The function `get_first_digit` returns an `Option<String>`.
fn get_first_digit(line: &str) -> Option<String> {
    let first_numeric_digit = find_first_numeric_digit(line);
    let first_alpha_digit = find_first_alpha_digit(line);

    match (first_numeric_digit, first_alpha_digit) {
        (Some((numeric_digit, numeric_idx)), Some((alpha_digit, alpha_idx))) => {
            if numeric_idx < alpha_idx {
                Some(numeric_digit)
            } else {
                alpha_to_numeric(&alpha_digit)
            }
        }
        (Some((numeric_digit, _)), None) => Some(numeric_digit),
        (None, Some((alpha_digit, _))) => alpha_to_numeric(&alpha_digit),
        _ => None,
    }
}

/// The function `get_last_digit` takes a string as input and returns the last numeric or alphabetic digit found in the
/// string, converting alphabetic digits to their corresponding numeric values if necessary.
///
/// Arguments:
///
/// * `line`: The `line` parameter is a string that represents a line of text.
///
/// Returns:
///
/// The function `get_last_digit` returns an `Option<String>`.
fn get_last_digit(line: &str) -> Option<String> {
    let last_numeric_digit = find_last_numeric_digit(line);
    let last_alpha_digit = find_last_alpha_digit(line);

    match (last_numeric_digit, last_alpha_digit) {
        (Some((numeric_digit, numeric_idx)), Some((alpha_digit, alpha_idx))) => {
            if numeric_idx > alpha_idx {
                Some(numeric_digit)
            } else {
                alpha_to_numeric(&alpha_digit)
            }
        }
        (Some((numeric_digit, _)), None) => Some(numeric_digit),
        (None, Some((alpha_digit, _))) => alpha_to_numeric(&alpha_digit),
        _ => None,
    }
}

/// The function `get_coord` extracts the first and last digit from a given string, concatenates them, and converts the
/// resulting string to an unsigned 32-bit integer.
///
/// Arguments:
///
/// * `line`: The `line` parameter is a `String` that represents a line of text.
///
/// Returns:
///
/// The function `get_coord` returns a `u32` value.
fn get_coord(line: String) -> u32 {
    let mut coord = String::new();

    if let Some(first_digit) = get_first_digit(&line) {
        coord = coord.add(&first_digit);
    }

    if let Some(last_digit) = get_last_digit(&line) {
        coord = coord.add(&last_digit);
    }

    assert!(coord.len() == 2);

    coord.parse::<u32>().unwrap()
}

/// The `get_input` function reads the contents of a file at the given path and returns them as a vector of strings, with
/// leading and trailing whitespace removed from each line.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `get_input` function is a `String` that represents the file path from which you
/// want to read the input.
///
/// Returns:
///
/// The function `get_input` returns a `Vec<String>`, which is a vector of strings.
fn get_input(path: String) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

/// The main function reads lines from a file, converts each line to a coordinate, and calculates the sum of all
/// coordinates.
fn main() {
    let path: String = "res/data.txt".to_string();
    let lines = get_input(path);
    let mut sum: u32 = 0;
    for line in lines {
        sum += get_coord(line);
    }

    println!("Sum is {}", sum);

    // println!("{:?}", get_coord("two1nine".to_string()));
    // println!("{:?}", get_coord("eightwothree".to_string()));
    // println!("{:?}", get_coord("abcone2threexyz".to_string()));
    // println!("{:?}", get_coord("xtwone3four".to_string()));
    // println!("{:?}", get_coord("4nineeightseven2".to_string()));
    // println!("{:?}", get_coord("zoneight234".to_string()));
    // println!("{:?}", get_coord("7pqrstsixteen".to_string()));
}
