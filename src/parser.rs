use std::io::Error;
use std::io::Read;
use std::fs::OpenOptions;

use item::Item;

pub fn parse(file_name: String) -> Result<Item, Error> {
    let lines = get_lines(&file_name);
    let root = Item::new(file_name);

    Ok(root)
}

pub fn get_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let mut file = OpenOptions::new().read(true).open(file_name);

    match file {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content);
            let t = content.split("\n").map(|t| t.to_owned()).collect::<Vec<String>>();
            Ok(t)
        },

        Err(e) => panic!("File not readable: {}", e),
    }
}

pub fn indent_levels(input_lines: Vec<String>) -> Vec<(usize, String)> {
    input_lines.into_iter().map(|line| {
        let iterator = line.chars();
        let whitespace: Vec<char> = iterator.take_while(|c| *c == ' ' || *c == '\t').collect();
        (whitespace.len(), line.trim().to_owned())
    }).collect::<Vec<(usize, String)>>()
}

mod test {
    use super::*;

    #[test]
    fn test_whitespace_counting() {
        let input = vec!("    4test".to_owned(), "\t\t2tabtest".to_owned(), "  2stest".to_owned());
        let expected_output: Vec<(usize, String)> = vec!((4, "4test".to_owned()), (2, "2tabtest".to_owned()), (2, "2stest".to_owned()));

        let actual_output = indent_levels(input);

        assert_eq!(actual_output, expected_output);
    }
}