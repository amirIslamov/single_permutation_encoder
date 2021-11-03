use std::io::{self, Error};
use std::num::ParseIntError;
use crate::decode::error::DecoderCreationError;
use crate::encode::error::EncoderCreationError;

pub enum KeyReadingError {
    IoError(io::Error),
    ParsingError(ParseIntError)
}

pub enum EncodingError {
    IoError(io::Error),
    KeyReadingError(KeyReadingError),
    EncoderCreationError(EncoderCreationError),
}

pub enum DecodingError {
    IoError(io::Error),
    KeyReadingError(KeyReadingError),
    DecoderCreationError(DecoderCreationError),
}

impl From<io::Error> for EncodingError {
    fn from(e: Error) -> Self {
        EncodingError::IoError(e)
    }
}

impl From<EncoderCreationError> for EncodingError {
    fn from(e: EncoderCreationError) -> Self {
        EncodingError::EncoderCreationError(e)
    }
}

impl From<io::Error> for DecodingError {
    fn from(e: Error) -> Self {
        DecodingError::IoError(e)
    }
}

impl From<DecoderCreationError> for DecodingError {
    fn from(e: DecoderCreationError) -> Self {
        DecodingError::DecoderCreationError(e)
    }
}

impl From<KeyReadingError> for DecodingError {
    fn from(e: KeyReadingError) -> Self {
        DecodingError::KeyReadingError(e)
    }
}

impl From<KeyReadingError> for EncodingError {
    fn from(e: KeyReadingError) -> Self {
        EncodingError::KeyReadingError(e)
    }
}

impl From<io::Error> for KeyReadingError {
    fn from(e: Error) -> Self {
        KeyReadingError::IoError(e)
    }
}

impl From<ParseIntError> for KeyReadingError {
    fn from(e: ParseIntError) -> Self {
        KeyReadingError::ParsingError(e)
    }
}