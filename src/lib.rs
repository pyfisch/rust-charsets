#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! The crate provides an enum representing all charset names used in Media Types
//! and HTTP header values. The list can be found at [the IANA Character Sets
//! registry](http://www.iana.org/assignments/character-sets/character-sets.xhtml).
//!
//! Charset names can be parsed from string, formatted to string and compared.
//! Charset names can be parsed from string, formatted to string and compared.
//! Unregistered charsets are represented using an `Unregistered` variant.

use std::fmt::{self, Display};
use std::str::FromStr;
use std::ascii::AsciiExt;
use std::error::Error as ErrorTrait;

pub use self::Charset::*;

/// An error type used for this crate.
///
/// It may be extended in the future to give more information.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Parsing as as charset failed.
    Invalid
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        return "The given charset is invalid"
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// Result type used for this library.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A Mime charset.
///
/// The string representation is normalised to upper case.
///
/// See http://www.iana.org/assignments/character-sets/character-sets.xhtml
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub enum Charset{
    /// US ASCII
    UsAscii,
    /// ISO-8859-1
    Iso88591,
    /// ISO-8859-2
    Iso88592,
    /// ISO-8859-3
    Iso88593,
    /// ISO-8859-4
    Iso88594,
    /// ISO-8859-5
    Iso88595,
    /// ISO-8859-6
    Iso88596,
    /// ISO-8859-7
    Iso88597,
    /// ISO-8859-8
    Iso88598,
    /// ISO-8859-9
    Iso88599,
    /// ISO-8859-10
    Iso885910,
    /// Shift_JIS
    ShiftJis,
    /// EUC-JP
    EucJp,
    /// ISO-2022-KR
    Iso2022Kr,
    /// EUC-KR
    EucKr,
    /// ISO-2022-JP
    Iso2022Jp,
    /// ISO-2022-JP-2
    Iso2022Jp2,
    /// ISO-8859-6-E
    Iso88596E,
    /// ISO-8859-6-I
    Iso88596I,
    /// ISO-8859-8-E
    Iso88598E,
    /// ISO-8859-8-I
    Iso88598I,
    /// GB2312
    Gb2312,
    /// Big5
    Big5,
    /// KOI8-R
    Koi8R,
    /// An arbitrary charset specified as a string
    Unregistered(String)
}

const MAPPING: [(Charset, &'static str); 24] = [
    (UsAscii, "US-ASCII"),
    (Iso88591, "ISO-8859-1"),
    (Iso88592, "ISO-8859-2"),
    (Iso88593, "ISO-8859-3"),
    (Iso88594, "ISO-8859-4"),
    (Iso88595, "ISO-8859-5"),
    (Iso88596, "ISO-8859-6"),
    (Iso88597, "ISO-8859-7"),
    (Iso88598, "ISO-8859-8"),
    (Iso88599, "ISO-8859-9"),
    (Iso885910, "ISO-8859-10"),
    (ShiftJis, "Shift-JIS"),
    (EucJp, "EUC-JP"),
    (Iso2022Kr, "ISO-2022-KR"),
    (EucKr, "EUC-KR"),
    (Iso2022Jp, "ISO-2022-JP"),
    (Iso2022Jp2, "ISO-2022-JP-2"),
    (Iso88596E, "ISO-8859-6-E"),
    (Iso88596I, "ISO-8859-6-I"),
    (Iso88598E, "ISO-8859-8-E"),
    (Iso88598I, "ISO-8859-8-I"),
    (Gb2312, "GB2312"),
    (Big5, "5"),
    (Koi8R, "KOI8-R")
];

impl Charset {
    fn name(&self) -> &str {
        if let &Unregistered(ref s) = self {
            return &s[..]
        }
        MAPPING.iter()
            .find(|&&(ref variant, _)| self == variant)
            .map(|&(_, name)| name).unwrap()
    }
}

impl Display for Charset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Charset {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<Charset> {
        Ok(MAPPING.iter()
            .find(|&&(_, ref name)| name.eq_ignore_ascii_case(s))
            .map(|&(ref variant, _)| variant.to_owned())
            .unwrap_or(Unregistered(s.to_owned())))
    }
}

impl PartialEq for Charset {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&UsAscii, &UsAscii) |
            (&Iso88591, &Iso88591) |
            (&Iso88592, &Iso88592) |
            (&Iso88593, &Iso88593) |
            (&Iso88594, &Iso88594) |
            (&Iso88595, &Iso88595) |
            (&Iso88596, &Iso88596) |
            (&Iso88597, &Iso88597) |
            (&Iso88598, &Iso88598) |
            (&Iso88599, &Iso88599) |
            (&Iso885910, &Iso885910) |
            (&ShiftJis, &ShiftJis) |
            (&EucJp, &EucJp) |
            (&Iso2022Kr, &Iso2022Kr) |
            (&EucKr, &EucKr) |
            (&Iso2022Jp, &Iso2022Jp) |
            (&Iso2022Jp2, &Iso2022Jp2) |
            (&Iso88596E, &Iso88596E) |
            (&Iso88596I, &Iso88596I) |
            (&Iso88598E, &Iso88598E) |
            (&Iso88598I, &Iso88598I) |
            (&Gb2312, &Gb2312) |
            (&Big5, &Big5) |
            (&Koi8R, &Koi8R) => true,
            (&Unregistered(ref s), &Unregistered(ref t)) => s.eq_ignore_ascii_case(t),
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(UsAscii,"us-ascii".parse().unwrap());
        assert_eq!(UsAscii,"US-Ascii".parse().unwrap());
        assert_eq!(UsAscii,"US-ASCII".parse().unwrap());
        assert_eq!(ShiftJis,"Shift-JIS".parse().unwrap());
        assert_eq!(Unregistered("ABCD".to_owned()),"abcd".parse().unwrap());
    }

    #[test]
    fn test_display() {
        assert_eq!("US-ASCII", format!("{}", UsAscii));
        assert_eq!("ABCD", format!("{}", Unregistered("ABCD".to_owned())));
    }

    #[test]
    fn test_cmp() {
        assert_eq!(Unregistered("foobar".to_owned()), Unregistered("FOOBAR".to_owned()));
    }
}
