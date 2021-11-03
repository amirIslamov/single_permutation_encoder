use clap::{App, Arg, SubCommand};
use single_permutation_encoder::cli::{Args, double_perm_decode, double_perm_encode, parse_config};
use single_permutation_encoder::cli::error::{DecodingError, EncodingError};

fn main() {
    let app = App::new("Double permutation encoder/decoder")
        .version("0.1.0")
        .author("amirIslamov")
        .about("Encodes/decodes bytes with double permutation cipher")
        .subcommand(
            SubCommand::with_name("encode")
                .about("encodes input file using a pair of keys")
                .version("0.1.0")
                .author("amirIslamov")
                .arg(Arg::new("input")
                    .short('i')
                    .about("sets input file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("INPUT_FILE"))
                .arg(Arg::new("output")
                    .short('o')
                    .about("sets output file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("OUTPUT_FILE"))
                .arg(Arg::new("key")
                    .short('k')
                    .about("sets key file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("KEY_FILE")))
        .subcommand(
            SubCommand::with_name("decode")
                .about("decodes input file using a pair of keys")
                .version("0.1.0")
                .author("amirIslamov")
                .arg(Arg::new("input")
                    .short('i')
                    .about("sets input file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("INPUT_FILE"))
                .arg(Arg::new("output")
                    .short('o')
                    .about("sets output file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("OUTPUT_FILE"))
                .arg(Arg::new("key")
                    .short('k')
                    .about("sets key file location")
                    .required(true)
                    .takes_value(true)
                    .value_name("KEY_FILE"))
        );

    let matches = app.get_matches();

    let args = parse_config(&matches);

    match args {
        Some(args) => match args {
            Args::Encode(config) => match double_perm_encode(config) {
                Ok(_) => {}
                Err(err) => match err {
                    EncodingError::IoError(io_err) =>
                        { eprintln!("An error occurred while reading one of provided files \nDetails: {} ", io_err) }
                    EncodingError::KeyReadingError(_) =>
                        { eprintln!("An error occurred while reading encryption key file \nDetails: Key does not correspond to the specified format") }
                    EncodingError::EncoderCreationError(_) =>
                        { eprintln!("An error occurred while creating an encoder \nDetails: Key is empty") }
                }
            },
            Args::Decode(config) => match double_perm_decode(config) {
                Ok(_) => {}
                Err(err) => match err {
                    DecodingError::IoError(io_err) =>
                        { eprintln!("An error occurred while reading one of provided files \nDetails: {} ", io_err) }
                    DecodingError::KeyReadingError(_) =>
                        { eprintln!("An error occurred while reading encryption key file \nDetails: Key does not correspond to the specified format") }
                    DecodingError::DecoderCreationError(_) =>
                        { eprintln!("An error occurred while creating a decoder \nDetails: Key is empty") }
                }
            }
        },
        None => eprintln!("No arguments specified; run with --help option to see list of possible arguments")
    }
}

