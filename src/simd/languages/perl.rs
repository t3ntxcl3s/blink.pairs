use crate::simd::*;
use matcher_macros::define_matcher;

define_matcher!(Perl {
    delimiters: [
        "(" => ")",
        "[" => "]",
        "{" => "}"
    ],
    line_comment: ["#"],
    string: ["\"", "'"],
});
