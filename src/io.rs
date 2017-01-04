use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::env::current_dir;

pub fn save_string_locally(content: String, filename: &str) -> Result<(), io::Error> {
    let mut local_filename = current_dir()
        .expect("Stop launching this without a PWD");

    local_filename.push(filename);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(local_filename);

    match file {
        Ok(mut file) => {
            match file.write(content.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_local_saving() {
        use std::fs::metadata;
        use std::env::current_dir;
        use std::fs::remove_file;

        save_string_locally("foobar".to_owned(), "temp").unwrap();

        let mut local_path = current_dir().unwrap();
        local_path.push("temp");

        assert!(metadata(&local_path).unwrap().is_file());
        remove_file(local_path);
    }
}