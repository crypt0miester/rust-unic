// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::Write;
use std::path::Path;

use reader::ucd::test::normalization_test::NORMALIZATION_TESTS;

use writer::utils::write;


pub fn generate(dir: &Path) {
    normalization_test_emit(&dir);
}


fn normalization_test_emit(dir: &Path) {
    let mut contents = "&[\n".to_owned();
    for test in NORMALIZATION_TESTS.0.iter() {
        contents.push_str("    (\"");

        for char in test.source.iter() {
            write!(contents, "{}", char.escape_unicode()).unwrap();
        }
        contents.push_str("\", \"");

        for char in test.nfc.iter() {
            write!(contents, "{}", char.escape_unicode()).unwrap();
        }
        contents.push_str("\", \"");

        for char in test.nfd.iter() {
            write!(contents, "{}", char.escape_unicode()).unwrap();
        }
        contents.push_str("\", \"");

        for char in test.nfkc.iter() {
            write!(contents, "{}", char.escape_unicode()).unwrap();
        }
        contents.push_str("\", \"");

        for char in test.nfkd.iter() {
            write!(contents, "{}", char.escape_unicode()).unwrap();
        }
        contents.push_str("\"),\n");
    }
    contents.push_str("]");

    write(&dir, "conformance_tests_data.rsv", &contents);
}
