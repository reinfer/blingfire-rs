mod errors;

use blingfire_sys::{
    FALimits_MaxArrSize as FA_LIMITS_MAX_ARRAY_SIZE, TextToSentences as text_to_sentences_ffi,
    TextToWords as text_to_words_ffi,
};
use failchain::ensure;
use std::{
    convert::TryInto,
    i32,
    os::raw::{c_char, c_int},
};

pub use crate::errors::{ErrorKind, Result};

pub const MAX_TEXT_LENGTH: usize = FA_LIMITS_MAX_ARRAY_SIZE as usize;

#[inline]
pub fn text_to_words(source: &str, destination: &mut String) -> Result<()> {
    tokenize(text_to_words_ffi, source, destination)
}

#[inline]
pub fn text_to_sentences(source: &str, destination: &mut String) -> Result<()> {
    tokenize(text_to_sentences_ffi, source, destination)
}

type Tokenizer = unsafe extern "C" fn(*const c_char, c_int, *mut c_char, c_int) -> c_int;

#[inline]
fn tokenize(tokenizer: Tokenizer, source: &str, destination: &mut String) -> Result<()>
where
{
    destination.clear();

    if source.is_empty() {
        return Ok(());
    }

    let source_len = source.len();
    ensure!(source_len <= MAX_TEXT_LENGTH, ErrorKind::SourceTooLarge);
    let source_len = source_len as c_int;

    loop {
        let length = unsafe {
            tokenizer(
                source.as_ptr() as *const c_char,
                source_len,
                destination.as_mut_ptr() as *mut c_char,
                destination.capacity().try_into().unwrap_or(i32::MAX),
            )
        };

        // The C++ function returned -1, an unknown error.
        ensure!(length > 0, ErrorKind::UnknownError);

        if length as usize > destination.capacity() {
            // There was not enough capacity in `destination` to store the parsed text.
            // Although the C++ function allocated an internal buffer with the parsed text, that's
            // not exposed. We'll have to reserve `length` bytes in `destination` (as
            // `destination.len() == 0`) and parse the `source` string again.
            destination.reserve_exact(length as usize);
            continue;
        } else {
            // The text was successfully parsed, set the length to the return value (-1 for the
            // null character).
            unsafe {
                destination.as_mut_vec().set_len(length as usize - 1);
            }
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{errors::ErrorKind, text_to_sentences, text_to_words, MAX_TEXT_LENGTH};

    const TEST_TEXT: &str = "I think. Sometimes, that my use of\ncommas, (and, occasionally, exclamation marks) can be excessive!!";
    const TEST_TEXT_WORDS: &str = "I think . Sometimes , that my use of commas , ( and , occasionally , exclamation marks ) can be excessive ! !";
    const TEST_TEXT_SENTENCES: &str = "I think.\nSometimes, that my use of commas, (and, occasionally, exclamation marks) can be excessive!!";

    #[test]
    fn text_to_words_new_string() {
        let mut parsed = String::new();
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
    }

    #[test]
    fn text_to_words_string_smaller_than_output() {
        let mut parsed = "hello".to_owned();
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
    }

    #[test]
    fn text_to_words_string_one_smaller_than_output() {
        // This test interesting due to the nul character.
        let mut parsed = String::with_capacity(TEST_TEXT_WORDS.len());
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
    }

    #[test]
    fn text_to_words_string_of_exactly_correct_size() {
        let mut parsed = String::with_capacity(TEST_TEXT_WORDS.len() + 1);
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
        assert_eq!(TEST_TEXT_WORDS.len() + 1, parsed.capacity());
    }

    #[test]
    fn text_to_words_string_of_larger_size() {
        let initial_capacity = TEST_TEXT_WORDS.len() + 10;
        let mut parsed = String::with_capacity(initial_capacity);
        parsed.push_str("uninitialised");
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
        assert_eq!(initial_capacity, parsed.capacity());
    }

    #[test]
    fn text_to_words_string_too_long() {
        let source = String::from_utf8(vec![b'.'; MAX_TEXT_LENGTH + 1]).unwrap();
        let mut destination = String::new();
        let result = text_to_words(&source, &mut destination);
        assert!(result.is_err() && *result.unwrap_err().kind() == ErrorKind::SourceTooLarge);
    }

    #[test]
    fn text_to_sentences_new_string() {
        let mut parsed = String::new();
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
    }

    #[test]
    fn text_to_sentences_string_smaller_than_output() {
        let mut parsed = "hello".to_owned();
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
    }

    #[test]
    fn text_to_sentences_string_one_smaller_than_output() {
        // This test interesting due to the nul character.
        let mut parsed = String::with_capacity(TEST_TEXT_SENTENCES.len());
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
    }

    #[test]
    fn text_to_sentences_string_of_exactly_correct_size() {
        let mut parsed = String::with_capacity(TEST_TEXT_SENTENCES.len() + 1);
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
        assert_eq!(TEST_TEXT_SENTENCES.len() + 1, parsed.capacity());
    }

    #[test]
    fn text_to_sentences_string_of_larger_size() {
        let initial_capacity = TEST_TEXT_SENTENCES.len() + 10;
        let mut parsed = String::with_capacity(initial_capacity);
        parsed.push_str("uninitialised");
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
        assert_eq!(initial_capacity, parsed.capacity());
    }
}
