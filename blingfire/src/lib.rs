mod errors;

use blingfire_sys::{TextToSentences as text_to_sentences_ffi, TextToWords as text_to_words_ffi};
use failchain::{bail, ResultExt};
use std::{convert::TryInto, mem};

pub use crate::errors::{ErrorKind, Result};

#[inline]
pub fn text_to_words(source: &str, destination: &mut String) -> Result<()> {
    tokenize(
        |raw_source, raw_source_len, raw_destination, raw_destination_capacity| unsafe {
            text_to_words_ffi(
                raw_source,
                raw_source_len,
                raw_destination,
                raw_destination_capacity,
            )
        },
        source,
        destination,
    )
}

#[inline]
pub fn text_to_sentences(source: &str, destination: &mut String) -> Result<()> {
    tokenize(
        |raw_source, raw_source_len, raw_destination, raw_destination_capacity| unsafe {
            text_to_sentences_ffi(
                raw_source,
                raw_source_len,
                raw_destination,
                raw_destination_capacity,
            )
        },
        source,
        destination,
    )
}

#[inline]
fn tokenize<Tokenizer>(tokenizer: Tokenizer, source: &str, destination: &mut String) -> Result<()>
where
    Tokenizer: Fn(
        *const std::os::raw::c_char,
        std::os::raw::c_int,
        *mut std::os::raw::c_char,
        std::os::raw::c_int,
    ) -> std::os::raw::c_int,
{
    destination.clear();

    if source.is_empty() {
        return Ok(());
    }

    loop {
        let length = tokenizer(
            source.as_ptr() as *const i8,
            source
                .len()
                .try_into()
                .chain_err(|| ErrorKind::SourceTooLarge)?,
            destination.as_mut_ptr() as *mut i8,
            destination
                .capacity()
                .try_into()
                .chain_err(|| ErrorKind::DestinationTooLarge)?,
        );

        if length < 0 {
            // The C++ function returned -1, an unknown error.
            bail!(ErrorKind::UnknownError);
        } else if length as usize > destination.capacity() {
            // There was not enough capacity in `destination` to store the parsed text.
            // Although the C++ function allocated an internal buffer with the parsed text, that's
            // not exposed. We'll have to to reserve `length` additional bytes in `destination` (as
            // `destination.len() == 0`) and parse the `source` string again.
            destination.reserve(length as usize);
            continue;
        } else {
            // The text was successfully parsed.

            // 1. Create a new string using the same buffer backing `destination` and with the
            //    `length` returned by the C++ function. N.B. the input was valid utf-8, so the
            //    parsed result will also be valid utf-8.
            let new_destination = unsafe {
                String::from_raw_parts(
                    destination.as_mut_ptr(),
                    length as usize - 1, // The C function adds a NULL character at the end.
                    destination.capacity(),
                )
            };

            // 2. Replace `destination` with the newly created string and ensure we don't run the
            //    destructors for original `destination` string.
            mem::forget(mem::replace(destination, new_destination));
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{text_to_sentences, text_to_words};
    const TEST_TEXT: &str = "I think. Sometimes, that my use of commas, (and, occasionally, exclamation marks) can be excessive!!";
    const TEST_TEXT_WORDS: &str = "I think . Sometimes , that my use of commas , ( and , occasionally , exclamation marks ) can be excessive ! !";
    const TEST_TEXT_SENTENCES: &str = "I think.\nSometimes, that my use of commas, (and, occasionally, exclamation marks) can be excessive!!";

    #[test]
    fn test_text_to_words() {
        let mut parsed = String::new();
        // Try with `parsed.capacity() == 0`
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());

        // Try when `parsed.capacity()` is sufficient for the output
        text_to_words(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_WORDS, parsed.as_str());
    }

    #[test]
    fn test_text_to_sentences() {
        let mut parsed = String::new();

        // Try with `parsed.capacity() == 0`
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());

        // Try when `parsed.capacity()` is sufficient for the output
        text_to_sentences(TEST_TEXT, &mut parsed).unwrap();
        assert_eq!(TEST_TEXT_SENTENCES, parsed.as_str());
    }
}
