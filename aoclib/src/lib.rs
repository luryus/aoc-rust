use itertools::{Either, Itertools};
use ndarray::{Array2, ArrayView2};
use num_integer::Integer;
use regex::Regex;
use std::io::{self, Read};
use std::iter::{repeat, Iterator};
use std::str::FromStr;

pub fn get_input_filename() -> Option<String> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        2 => args.into_iter().nth(1),
        1 => None,
        _ => panic!("Invalid number of arguments ({})", args.len() - 1),
    }
}

pub fn read_input_string() -> io::Result<String> {
    match get_input_filename() {
        Some(path) => std::fs::read_to_string(path),
        None => read_stdin_to_string(),
    }
}

pub fn read_stdin_to_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.to_owned())
}

pub fn read_input_lines() -> io::Result<Vec<String>> {
    match get_input_filename() {
        Some(path) => read_file_lines(&path),
        None => read_stdin_lines(),
    }
}

pub fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input.lines().map(|l| l.to_owned()).collect())
}

pub fn read_stdin_lines() -> io::Result<Vec<String>> {
    let input = read_stdin_to_string()?;
    Ok(input.lines().map(|l| l.to_owned()).collect())
}

pub fn read_input_ints<T: Integer + FromStr>(signed: bool) -> io::Result<Vec<T>> {
    match get_input_filename() {
        Some(path) => read_ints_from_file(&path, signed),
        None => read_ints_from_stdin(signed),
    }
}

pub fn read_ints_from_stdin<T: Integer + FromStr>(signed: bool) -> io::Result<Vec<T>> {
    let s = read_stdin_to_string()?;
    Ok(read_ints_from_string(&s, signed))
}

pub fn read_ints_from_file<T: Integer + FromStr>(
    filename: &str,
    signed: bool,
) -> io::Result<Vec<T>> {
    let s = std::fs::read_to_string(filename)?;
    Ok(read_ints_from_string(&s, signed))
}

pub fn read_ints_from_string<T: Integer + FromStr>(s: &str, signed: bool) -> Vec<T> {
    let re = Regex::new(if signed { r"-?\d+" } else { r"\d+" }).unwrap();
    re.find_iter(s)
        .map(|m| m.as_str())
        .filter_map(|m| m.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn read_input_regex_matches(regex_pattern: &str) -> io::Result<Vec<String>> {
    match get_input_filename() {
        Some(path) => read_regex_matches_from_file(&path, regex_pattern),
        None => read_regex_matches_from_stdin(regex_pattern),
    }
}

pub fn read_regex_matches_from_stdin(regex_pattern: &str) -> io::Result<Vec<String>> {
    let s = read_stdin_to_string()?;
    let matches = read_regex_matches_from_string(&s, regex_pattern);

    let res = matches.into_iter().map(|sm| sm.to_owned()).collect();

    Ok(res)
}

pub fn read_regex_matches_from_file(
    filename: &str,
    regex_pattern: &str,
) -> io::Result<Vec<String>> {
    let s = std::fs::read_to_string(filename)?;
    let matches = read_regex_matches_from_string(&s, regex_pattern);

    let res = matches.into_iter().map(|sm| sm.to_owned()).collect();

    Ok(res)
}

pub fn read_regex_matches_from_string<'a>(s: &'a str, regex_pattern: &str) -> Vec<&'a str> {
    let re = Regex::new(regex_pattern).unwrap();
    re.find_iter(s).map(|m| m.as_str()).collect()
}

pub fn read_input_char_matrix() -> io::Result<Array2<char>> {
    let lines = read_input_lines()?;
    let h = lines.len();
    let w = lines[0].len();

    Array2::from_shape_vec((h, w), lines.iter().flat_map(|l| l.chars()).collect())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn read_file_char_matrix(filename: &str) -> io::Result<Array2<char>> {
    let lines = read_file_lines(filename)?;
    let h = lines.len();
    let w = lines[0].len();

    Array2::from_shape_vec((h, w), lines.iter().flat_map(|l| l.chars()).collect())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn read_string_char_matrix(str: &str) -> io::Result<Array2<char>> {
    let lines: Vec<_> = str.lines().collect();
    let h = lines.len();
    let w = lines.iter().map(|l| l.len()).max().unwrap();

    Array2::from_shape_vec(
        (h, w),
        lines
            .iter()
            .flat_map(|l| {
                if l.len() < w {
                    Either::Left(
                        l.chars()
                            .chain(repeat(Default::default()).take(w - l.len())),
                    )
                } else {
                    Either::Right(l.chars())
                }
            })
            .collect(),
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn read_input_byte_matrix() -> io::Result<Array2<u8>> {
    let lines = read_input_lines()?;
    let h = lines.len();
    let w = lines[0].len();

    Array2::from_shape_vec((h, w), lines.iter().flat_map(|l| l.bytes()).collect())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn read_file_byte_matrix(filename: &str) -> io::Result<Array2<u8>> {
    let lines = read_file_lines(filename)?;
    let h = lines.len();
    let w = lines[0].len();

    Array2::from_shape_vec((h, w), lines.iter().flat_map(|l| l.bytes()).collect())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn read_input_int_matrix<T: Integer + From<u32>>() -> io::Result<Array2<T>> {
    let cm = read_input_char_matrix()?;
    if !cm.iter().all(|&c| c.is_ascii_digit()) {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Matrix char not a digit",
        ))
    } else {
        Ok(cm.map(|&c| c.to_digit(10).unwrap().into()))
    }
}

pub fn read_file_int_matrix<T: Integer + From<u32>>(filename: &str) -> io::Result<Array2<T>> {
    let cm = read_file_char_matrix(filename)?;
    if !cm.iter().all(|&c| c.is_ascii_digit()) {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Matrix char not a digit",
        ))
    } else {
        Ok(cm.map(|&c| c.to_digit(10).unwrap().into()))
    }
}

pub fn split_to_tuple2<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    s.split_once(pattern)
}

pub fn split_to_tuple3<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str, &'a str)> {
    let parts = s.splitn(3, pattern);
    parts.collect_tuple()
}

pub fn split_to_tuple4<'a>(
    s: &'a str,
    pattern: &str,
) -> Option<(&'a str, &'a str, &'a str, &'a str)> {
    let parts = s.splitn(4, pattern);
    parts.collect_tuple()
}

#[test]
fn test_read_ints_from_string() {
    let s = "a123b22 123x02\n123-22";
    let res: Vec<i32> = read_ints_from_string(s, true);

    assert_eq!(vec![123i32, 22, 123, 2, 123, -22], res);
}

#[test]
fn test_read_regex_matches_from_string() {
    let s = "0.12,1.23,4.2\n111.1,.,111.,.23";
    let re = r"\d+\.\d+";

    let m = read_regex_matches_from_string(s, re);
    assert_eq!(vec!["0.12", "1.23", "4.2", "111.1"], m);
}

pub trait UnwrapOptionIterator<T> {
    type Output: Iterator<Item = T>;
    fn unwrap_options(self) -> Self::Output;
}

impl<T, I: Iterator<Item = Option<T>>> UnwrapOptionIterator<T> for I {
    type Output = std::iter::Map<Self, fn(Option<T>) -> T>;

    fn unwrap_options(self) -> Self::Output {
        self.map(|x| x.unwrap())
    }
}

pub fn make_2d_array<T>(v: Vec<Vec<T>>) -> Option<Array2<T>> {
    let ncols = v[0].len();
    let nrows = v.len();

    Array2::from_shape_vec((ncols, nrows), v.into_iter().flatten().collect_vec()).ok()
}

pub fn print_bool_matrix<T: Default + PartialEq>(mtx: &Vec<Vec<T>>) {
    let def = T::default();
    for r in mtx {
        println!(
            "{}",
            r.iter()
                .map(|c| if &def != c { '█' } else { '.' })
                .collect::<String>()
        );
    }
}

pub fn print_bool_ndarray<T: Default + PartialEq>(mtx: ArrayView2<T>) {
    let def = T::default();
    for r in mtx.rows() {
        println!(
            "{}",
            r.iter()
                .map(|c| if &def != c { '█' } else { '.' })
                .collect::<String>()
        );
    }
}

pub fn print_char_ndarray(mtx: ArrayView2<char>) {
    for r in mtx.rows() {
        println!("{}", r.iter().collect::<String>());
    }
}

#[macro_export]
macro_rules! get_test_input_file {
    ($n:expr) => {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bin")
            .join("inputs")
            .join(format!("{:02}.txt", $n))
            .to_str()
            .unwrap()
    };
}

pub mod iter {
    pub struct TakeUntilInclusive<I, P> {
        inner: I,
        predicate: P,
        fired: bool,
    }

    impl<I, P> Iterator for TakeUntilInclusive<I, P>
    where
        I: Iterator,
        P: Fn(&<I as Iterator>::Item) -> bool,
    {
        type Item = <I as Iterator>::Item;

        fn next(&mut self) -> Option<Self::Item> {
            if self.fired {
                return None;
            }

            if let Some(val) = self.inner.next() {
                if (self.predicate)(&val) {
                    self.fired = true;
                }
                return Some(val);
            }
            None
        }
    }

    pub trait TakeUntilInclusiveExt: Iterator {
        fn take_until_inclusive<P>(self, predicate: P) -> TakeUntilInclusive<Self, P>
        where
            P: Fn(&Self::Item) -> bool,
            Self: Sized,
        {
            TakeUntilInclusive {
                inner: self,
                predicate,
                fired: false,
            }
        }
    }

    impl<I: Iterator> TakeUntilInclusiveExt for I {}

    #[cfg(test)]
    mod test {
        use super::TakeUntilInclusiveExt;

        #[test]
        fn test_take_until_inclusive() {
            let res: Vec<_> = (0..5).take_until_inclusive(|v| *v > 2).collect();
            assert_eq!(vec![0, 1, 2, 3], res);

            let res: Vec<_> = [0, 0, 1, 0, 0]
                .into_iter()
                .take_until_inclusive(|v| *v == 1)
                .collect();
            assert_eq!(vec![0, 0, 1], res);
        }
    }
}
