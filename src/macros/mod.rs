#[macro_export]
macro_rules! merge_chars_range {
    ($i1:ident, $i2:ident) => {
        {
            let mut lhs: SrcCharsRange = $i1;
            let rhs: SrcCharsRange = $i2;
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
            let chars_range_vec: Vec<SrcCharsRange> = $e;
            let mut lhs: SrcCharsRange = $i1;
            for char_range in &chars_range_vec {
                lhs.merge(char_range)
            }
            lhs
        }
    };
    ($e:expr) => {
        {
            let chars_range_vec: Vec<SrcCharsRange> = $e;
            let mut chars_range = chars_range_vec[0];

            for i in (1..chars_range_vec.len()) {
                chars_range.merge(&chars_range_vec[i]);
            }
            chars_range
        }
    };
    ($e1:expr, $e2:expr) => {
        {
            let mut chars_range: SrcCharsRange = $e1;
            let other: SrcCharsRange = $e2;
            chars_range.merge(&other);
            chars_range
        }
    };
}

#[macro_export]
macro_rules! def_binary_val_method {
    (
        $method_name:ident,
        | $lhs:ident $op:tt $rhs:ident | -> Self,
        { $(($lhs_type:ident, $rhs_type:ident) $(: $e:expr)?),+ $(,)? }
    ) => {
        
        pub fn $method_name(&self, other: &Self) -> Result<Self, String> {

            match (self, other) {
                $(
                    (Self::$lhs_type(lhs), Self::$rhs_type(rhs)) => {
                        let $lhs = lhs;
                        let $rhs = rhs;
                        Ok(Self::$lhs_type(def_binary_val_method!(@calc $lhs $op $rhs $($e)?)))
                    },
                   
                )+
                _ => Err(format!("Operator '{}' is not defined for {} and {}", stringify!($op), self.dissasemble(), other.dissasemble())),
            }
        }
    };

    (
        $method_name:ident,
        | $_1:ident $op:tt $_2:ident | -> $ret_type:ident,
        { $(($lhs_type:ident, $rhs_type:ident)),+ $(,)? }
    ) => {
        pub fn $method_name(&self, other: &Self) -> Result<Self, String> {
            match (self, other) {
                $(
                    (Self::$lhs_type(lhs), Self::$rhs_type(rhs)) => Ok(Self::$ret_type(lhs $op rhs)),
                )+
                _ => Err(format!("Operator '{}' is not defined for {} and {}", stringify!($op), self.dissasemble(), other.dissasemble())),
            }
        }
    };

    (@calc $lhs:ident $op:tt $rhs:ident $e:expr) => {
        $e
    };

    (@calc $lhs:ident $op:tt $rhs:ident) => {
        $lhs $op $rhs
    };
}

#[macro_export]
macro_rules! define_unary_val_method {
    (
        $method_name:ident,
        | $op:tt $rhs:ident | -> Self,
        { $(($rhs_type:ident) $(: $e:expr)?),+ $(,)? }
    ) => {
        pub fn $method_name(&self) -> Result<Self, String> {
            match self {
                $(
                  Self::$rhs_type(rhs) => {
                    let $rhs = rhs;
                    Ok(Self::$rhs_type(define_unary_val_method!(@calc $op $rhs $($e)?)))
                  }
                )+
                _ => Err(format!("Operator '{}' is not defined for {}", stringify!($op), self.dissasemble()))
            }
        }
    };

    (
        $method_name:ident,
        | $op:tt $rhs:ident | -> $ret_type:ident,
        { $(($rhs_type:ident) $(: $e:expr)?),+ $(,)? }
    ) => {
        pub fn $method_name(&self) -> Result<Self, String> {
            match self {
                $(
                  Self::$rhs_type(rhs) => {
                    let $rhs = rhs;
                    Ok(Self::$ret_type(define_unary_val_method!(@calc $op $rhs $($e)?)))
                  }
                )+
                _ => Err(format!("Operator '{}' is not defined for {}", stringify!($op), self.dissasemble()))
            }
        }
    };

    (@calc $op:tt $rhs:ident $e:expr) => {
        $e
    };

    (@calc $op:tt $rhs:ident) => {
        $op $rhs
    };
}

#[macro_export]
macro_rules! create_tokens_and_parse_rules {
    ($([$tokentype:ident] = { $prefix:ident, $infix:ident, $prec:ident }),+ $(,)?) => {
        use crate::parser::precedence::Precedence;
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum TokenType {
            $(
                $tokentype = ${index()},
            )+
        }

        impl From<TokenType> for usize {
            fn from(value: TokenType) -> Self {
                match value {
                    $(
                        TokenType::$tokentype => ${index()},
                    )+
                }
            }
        }

        pub const PARSE_RULES: [ParseRule; ${count($tokentype)}] = [
            $(
                ParseRule {
                    prefix: create_tokens_and_parse_rules!(@construct_rule $prefix),
                    infix: create_tokens_and_parse_rules!(@construct_rule $infix),
                    precedence: create_tokens_and_parse_rules!(@make_precedence $prec)
                },
            )+
        ];
    };

    (@make_precedence $prec:ident) => {
        Precedence::$prec
    };

    (@construct_rule None) => {
        None
    };

    (@construct_rule $method_name:ident) => {
        Some(|c, arg| c.$method_name(arg))
    };
}
