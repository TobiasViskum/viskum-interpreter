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

macro_rules! def_unary_val_method {
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
                    Ok(Self::$rhs_type(def_unary_val_method!(@calc $op $rhs $($e)?)))
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
                    Ok(Self::$ret_type(def_unary_val_method!(@calc $op $rhs $($e)?)))
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

pub(crate) use def_binary_val_method;
pub(crate) use def_unary_val_method;

macro_rules! def_binary_try_method {
    (
        $method_name:ident,
        $op:expr => $ret_type:ident,
        { $(($lhs_type:ident, $rhs_type:ident)),+ $(,)? }
    ) => {
        
        fn $method_name(&self, other: &Self) -> Result<Self, String> {
            let minified_lhs = Self::get_minified(self);
            let minified_rhs = Self::get_minified(other);

            match (minified_lhs, minified_rhs) {
                $(
                    (Self::$lhs_type, Self::$rhs_type) => {
                        Ok(Self::$ret_type)
                    },
                   
                )+
                (lhs, rhs) => Err(format!("Operator `{}` is not defined for `{}` and `{}`", $op.dissasemble(), lhs.dissasemble(), rhs.dissasemble())),
            }
        }
    };

    ($method_name:ident, $op:expr, { $(($lhs_type:ident, $rhs_type:ident)),+ $(,)? }) => {
        
        pub fn $method_name(&self, other: &Self) -> Result<Self, String> {
            let minified_lhs = Self::get_minified(self);
            let minified_rhs = Self::get_minified(other);

            match (minified_lhs, minified_rhs) {
                $(
                    (Self::$lhs_type, Self::$rhs_type) => {
                        Ok(Self::$lhs_type)
                    },
                   
                )+
                (lhs, rhs) => Err(format!("Operator `{}` is not defined for `{}` and `{}`", $op.dissasemble(), lhs.dissasemble(), rhs.dissasemble())),
            }
        }
    };
}

macro_rules! def_unary_try_method {
    ($method_name:ident, $op:expr => $ret_type:ident, { $($rhs_type:ident),+ $(,)? }) => {
        pub fn $method_name(&self) -> Result<Self, String> {
            let minified_rhs = Self::get_minified(self);
            match minified_rhs {
                $(
                  Self::$rhs_type => {
                    Ok(Self::$ret_type)
                  }
                )+
                _ => Err(format!("Operator `{}` is not defined for `{}`", $op.dissasemble(), self.dissasemble()))
            }
        }
    };

    ($method_name:ident, $op:expr, { $($rhs_type:ident),+ $(,)? }) => {
        pub fn $method_name(&self) -> Result<Self, String> {
            let minified_rhs = Self::get_minified(self);
            match minified_rhs {
                $(
                  Self::$rhs_type => {
                    Ok(Self::$rhs_type)
                  }
                )+
                _ => Err(format!("Operator `{}` is not defined for `{}`", $op.dissasemble(), self.dissasemble()))
            }
        }
    };
}

pub(crate) use def_binary_try_method;
pub(crate) use def_unary_try_method;
