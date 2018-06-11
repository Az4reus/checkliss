use clap::App;
use clap::Arg;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn parse() -> Config {
    let matches = App::new("checklist")
        .about("Turns plaintext lists into printable checklist pdfs.")
        .author("Mordecai Malignatus <mordecai@malignat.us>")
        .version("0.1")
        .arg(
            Arg::with_name("source_file")
                .required(true)
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::with_name("target_file")
                .short("o")
                .long("target")
                .help("File in which to save the end result."),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Enable additional log messages for debugging"),
        )
        .arg(
            Arg::with_name("keep_tex")
                .short("k")
                .long("keep-tex")
                .help("Do not delete the tex file used to generate the PDF"),
        )
        .get_matches();

    to_config(matches)
}

fn to_config(matches: ArgMatches) -> Config {
    let v = matches.is_present("v");
    let keep = matches.is_present("keep_tex");
    let source_file_string = matches.value_of("source_file").unwrap();
    let source_file = PathBuf::from(source_file_string.to_owned());
    let target_file = match matches.is_present("target_file") {
        true => {
            let str = matches
                .value_of("target_file")
                .expect("You will have to specify a target file.");
            PathBuf::from(str.to_owned())
        }
        false => source_file.clone(),
    };

    Config {
        verbose: v,
        keep_tex: keep,
        target_file: target_file,
        source_file: source_file,
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub keep_tex: bool,
    pub target_file: PathBuf,
    pub source_file: PathBuf,
}
