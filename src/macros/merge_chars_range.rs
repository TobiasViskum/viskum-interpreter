macro_rules! merge_chars_range {
    ($i1:ident, $i2:ident) => {
        {
            let mut lhs: crate::compiler::error_handler::SrcCharsRange = $i1;
            let rhs: crate::compiler::error_handler::SrcCharsRange = $i2;
            lhs.merge(&rhs);
            lhs
        }
    };
    ($i1:ident, $i2:ident, $($idents:ident),+) => {
        {
            let merged = merge_chars_range!($i1, $i2);
            merge_chars_range!(merged, $($idents),+)
        }
    };
    ($i1:ident, $e:expr) => {
        {
            let chars_range_vec: Vec<crate::compiler::error_handler::SrcCharsRange> = $e;
            let mut lhs: crate::compiler::error_handler::SrcCharsRange = $i1;
            for char_range in &chars_range_vec {
                lhs.merge(char_range)
            }
            lhs
        }
    };
    ($e:expr) => {
        {
            let chars_range_vec: Vec<crate::compiler::error_handler::SrcCharsRange> = $e;
            let mut chars_range = chars_range_vec[0];

            for i in (1..chars_range_vec.len()) {
                chars_range.merge(&chars_range_vec[i]);
            }
            chars_range
        }
    };
    ($e1:expr, $e2:expr) => {
        {
            let mut chars_range: crate::compiler::error_handler::SrcCharsRange = $e1;
            let other: crate::compiler::error_handler::SrcCharsRange = $e2;
            chars_range.merge(&other);
            chars_range
        }
    };
}

pub(crate) use merge_chars_range;
