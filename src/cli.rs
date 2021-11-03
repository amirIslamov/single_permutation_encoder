use std::path::Path;
use clap::ArgMatches;
use crate::encode::DoublePermutationEncoded;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use crate::decode::DoublePermutationDecoded;
use crate::cli::error::{DecodingError, EncodingError, KeyReadingError};


pub mod error;

pub enum Args<'a> {
    Encode(Conf<'a>),
    Decode(Conf<'a>)
}

pub struct Conf<'a> {
    inp: &'a Path,
    out: &'a Path,
    hkey: &'a Path,
    vkey: &'a Path
}

pub fn double_perm_encode(cfg: Conf) -> Result<(), EncodingError> {
    let input = fs::read(cfg.inp)?;
    let hkey = read_key(cfg.hkey)?;
    let vkey = read_key(cfg.vkey)?;
    let encoded =
        DoublePermutationEncoded::new(&input, &hkey, &vkey)?;

    let output: Vec<u8> = encoded.into_iter().collect();
    fs::write(cfg.out, output)?;

    Ok(())
}

pub fn double_perm_decode(cfg: Conf) -> Result<(), DecodingError> {
    let input = fs::read(cfg.inp)?;
    let hkey = read_key(cfg.hkey)?;
    let vkey = read_key(cfg.vkey)?;
    let encoded = DoublePermutationDecoded::new(&input, &hkey, &vkey)?;

    let output: Vec<u8> = encoded.into_iter().collect();
    fs::write(cfg.out, output)?;

    Ok(())
}

pub fn parse_config(matches: &ArgMatches) -> Option<Args> {
    let encode_matches =  matches.subcommand_matches("encode");
    if encode_matches.is_some() {
        let inp: &str = matches.value_of("input")?;
        let out: &str = matches.value_of("output")?;
        let hkey: &str = matches.value_of("hkey")?;
        let vkey: &str = matches.value_of("vkey")?;

        return Some(Args::Encode(Conf {
            inp: Path::new(inp), out: Path::new(out),
            hkey: Path::new(hkey), vkey: Path::new(vkey)
        }))
    }

    let decode_matches = matches.subcommand_matches("decode");
    if decode_matches.is_some() {
        let inp: &str = matches.value_of("input")?;
        let out: &str = matches.value_of("output")?;
        let hkey: &str = matches.value_of("hkey")?;
        let vkey: &str = matches.value_of("vkey")?;

        return Some(Args::Decode(Conf {
            inp: Path::new(inp), out: Path::new(out),
            hkey: Path::new(hkey), vkey: Path::new(vkey)
        }))
    }

    None
}

pub fn read_key(p: &Path) -> Result<Vec<usize>, KeyReadingError> {
    let mut file = File::open(p)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let key_result: Result<Vec<usize>, ParseIntError> =
        contents.split_whitespace().map(|x| x.parse()).collect();

    let key = key_result?;

    Ok(key)
}