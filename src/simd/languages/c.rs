use crate::simd::*;
use matcher_macros::define_matcher;

define_matcher!(C {
    delimiters: [
        "(" => ")",
        "[" => "]",
        "{" => "}"
    ],
    line_comment: ["//"],
    block_comment: ["/*" => "*/"],
    string: ["\""],
    char: ["'"],
    block_string: []
});
