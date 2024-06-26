macro_rules! create_tokens_and_parse_rules {
    ($([$tokentype:ident] = { $prefix:ident, $infix:ident, $prec:ident }),+ $(,)?) => {
        use crate::compiler::parser::precedence::Precedence;
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

        pub static PARSE_RULES: [ParseRule; ${count($tokentype)}] = [
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
}

pub(crate) use create_tokens_and_parse_rules;
