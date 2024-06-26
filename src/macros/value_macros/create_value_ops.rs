macro_rules! create_value_ops {
    (
        $(
            $enum_name:ident($($variant:ident $(($nested_enum:ident),)? $($symbol:literal,)?)+)
        ),+
        $(,)?
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum $enum_name {
                $(
                    $variant $(
                        ($nested_enum)
                    )? ,
                )+
            }

            impl $enum_name {
                pub fn get_op_len(&self) -> usize {
                    match self {
                        $(
                            $(
                                Self::$variant => $symbol.len(),
                            )?
                            $(
                                Self::$variant(nested_enum) => {
                                    let nested_enum = nested_enum as &$nested_enum;
                                    nested_enum.get_op_len()
                                },
                            )?
                        )+

                    }
                }
            }

            impl crate::Dissasemble for $enum_name {
                fn dissasemble(&self) -> String {
                    match self {
                        $(
                            $(
                                Self::$variant => $symbol.to_string(),
                            )?
                            $(
                                Self::$variant(nested_enum) => {
                                    let nested_enum = nested_enum as &$nested_enum;
                                    nested_enum.dissasemble()
                                },
                            )?
                        )+
                    }
                }
            }
        )+
    };
}
pub(crate) use create_value_ops;
