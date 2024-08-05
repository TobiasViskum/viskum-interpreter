macro_rules! create_tokens_and_parse_rules {
    ($([$tokentype:ident] = { $prefix:ident, $infix:ident, $prec:ident }),+ $(,)?) => {
        use crate::compiler::parser::precedence::Precedence;
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum TokenType {
            $(
                $tokentype,
            )+
        }

        impl From<TokenType> for usize {
            fn from(value: TokenType) -> Self {
                let mut index = 0;
                match value {
                    $(
                        TokenType::$tokentype => {
                            index += 1;
                            index
                        }
                        
                    )+
                }
            }
        }

        pub static PARSE_RULES: [ParseRule; create_tokens_and_parse_rules!(@count $($tokentype),+)] = [
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
        Some(|c, arg, arena| c.$method_name(arg, arena))
    };

    (@count $($token:ident),*) => {
        [$(stringify!($token)),*].len()
    };

    (@index $($token:ident),*; $target:ident) => {
        {
            let mut index = 0;
            $(
                if stringify!($token) == stringify!($target) {
                    break;
                }
                index += 1;
            )*
            index
        }
    };
}

pub(crate) use create_tokens_and_parse_rules;
