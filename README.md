# rust-charsets
[![Build Status](https://travis-ci.org/pyfisch/rust-charsets.svg?branch=master)](https://travis-ci.org/pyfisch/rust-charsets)
[![Coverage Status](https://coveralls.io/repos/pyfisch/rust-charsets/badge.svg)](https://coveralls.io/r/pyfisch/rust-charsets)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/rust-charsets)](https://crates.io/crates/rust-charsets)

The documentation is located at http://pyfisch.github.io/rust-charsets/

The crate provides an enum representing all charset names used in Media Types
and HTTP header values. The list can be found at [the IANA Character Sets
registry](http://www.iana.org/assignments/character-sets/character-sets.xhtml).

Charset names can be parsed from string, formatted to string and compared.
Unregistered charsets are represented useing an `Unregistered` variant.
