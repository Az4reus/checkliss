use std::io::Error;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use item::Item;
use opts::Config;

pub fn parse(config: &Config) -> Result<Item, Error> {
    parse_file(&config.source_file)
}

fn parse_file(file: &PathBuf) -> Result<Item, Error> {
    let mut source_file_handle: File = OpenOptions::new()
        .read(true)
        .open(file)
        .expect("Opening source file failed.");

    let mut source_file_contents = String::new();
    source_file_handle.read_to_string(&mut source_file_contents)?;

    parse_text(source_file_contents)
}

fn parse_text(text: String) -> Result<Item, Error> {
    let line_items = text.split("\n")
        .filter(|x| x != &"")
        .map(|x| Item::new(x.to_owned()))
        .collect::<Vec<Item>>();
    Ok(Item {
        title: String::new(),
        children: line_items,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sublist_parsing() {
        let text = r#"
        - Item 1
        - Item 2
            - SubItem 1
            - SubItem 2
        "#.to_owned();

        let expected_sublist = vec![
            Item::new("- SubItem 1".to_owned()), 
            Item::new("- SubItem 2".to_owned()),
        ];

        let expected_children = vec![
            Item::new("- Item 1".to_owned()), 
            Item::new("- Item 2".to_owned()).set_children(expected_sublist),
        ];

        let expected_root = Item {
            title: String::new(), 
            children: expected_children,
        };

        assert_eq!(parse_text(text).unwrap(), expected_root)
    }

    #[test]
    fn test_very_basic_parsing() {
        let test = r#"
- Test 1
- Test 2
Test 3
- Test 4
    - Test 5"#.to_owned();

        let expected_items = vec![
            Item::new("- Test 1".to_owned()),
            Item::new("- Test 2".to_owned()),
            Item::new("Test 3".to_owned()),
            Item::new("- Test 4".to_owned()),
            Item::new("- Test 5".to_owned()),
        ];

        let expected_root = Item {
            title: String::new(),
            children: expected_items,
        };

        assert_eq!(parse_text(test).unwrap(), expected_root)
    }
}
