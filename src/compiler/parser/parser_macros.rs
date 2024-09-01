macro_rules! previous {
    ($self:ident, metadata) => {
        $self.get_previous().get_metadata()
    };
    ($self:ident, lexeme) => {
        $self.get_previous().get_lexeme($self.source)
    };
    ($self:ident, ttype) => {
        $self.get_previous().get_ttype()
    };
    ($self:ident, line) => {
        $self.get_previous().get_line()
    };
    ($self:ident, msg) => {
        $self.get_previous().get_message()
    };
    ($self:ident $(, $keyword:ident)+) => {
        ($(previous!($self, $keyword), )+)
    };
}
pub(in crate::compiler::parser) use previous;

macro_rules! current {
    ($self:ident, metadata) => {
        $self.get_current().get_metadata()
    };
    ($self:ident, lexeme) => {
        $self.get_current().get_lexeme($self.source)
    };
    ($self:ident, ttype) => {
        $self.get_current().get_ttype()
    };
    ($self:ident, line) => {
        $self.get_current().get_line()
    };
    ($self:ident, msg) => {
        $self.get_current().get_message()
    };
    ($self:ident $(, $keyword:ident)+) => {
        ($(current!($self, $keyword), )+)
    };
}
pub(in crate::compiler::parser) use current;

macro_rules! next {
    ($self:ident, metadata) => {
        $self.get_next().map(|token| token.get_metadata())
    };
    ($self:ident, lexeme) => {
        $self.get_next().map(|token| token.get_lexeme($self.source))
    };
    ($self:ident, ttype) => {
        $self.get_next().map(|token| token.get_ttype())
    };
    ($self:ident, line) => {
        $self.get_next().map(|token| token.get_line())
    };
    ($self:ident, msg) => {
        $self.get_next().map(|token| token.get_message())
    };
    ($self:ident $(, $keyword:ident)+) => {
        ($(next!($self, $keyword), )+)
    };
}
pub(in crate::compiler::parser) use next;

macro_rules! peek {
    ($self:ident, $i:ident, metadata) => {
        $self.peek($i).map(|token| token.get_metadata())
    };
    ($self:ident, $i:ident, lexeme) => {
        $self.peek($i).map(|token| token.get_lexeme($self.source))
    };
    ($self:ident, $i:ident, ttype) => {
        $self.peek($i).map(|token| token.get_ttype())
    };
    ($self:ident, $i:ident, line) => {
        $self.peek($i).map(|token| token.get_line())
    };
    ($self:ident, $i:ident, msg) => {
        $self.peek($i).map(|token| token.get_message())
    };
    ($self:ident, $i:ident $(, $keyword:ident)+) => {
        {
            let result = ($(peek!($self, $i, $keyword), )+);

            if result.0.is_some() {
                Some(($(peek!($self, $i, $keyword).unwrap(), )+))
            } else {
                None
            }
        }
    };

    ($self:ident, $i:literal, metadata) => {
        $self.peek($i).map(|token| token.get_metadata())
    };
    ($self:ident, $i:literal, lexeme) => {
        $self.peek($i).map(|token| token.get_lexeme($self.source))
    };
    ($self:ident, $i:literal, ttype) => {
        $self.peek($i).map(|token| token.get_ttype())
    };
    ($self:ident, $i:literal, line) => {
        $self.peek($i).map(|token| token.get_line())
    };
    ($self:ident, $i:literal, msg) => {
        $self.peek($i).map(|token| token.get_message())
    };
    ($self:ident, $i:literal $(, $keyword:ident)+) => {
        {
            let result = ($(peek!($self, $i, $keyword), )+);

            if result.0.is_some() {
                Some(($(peek!($self, $i, $keyword).unwrap(), )+))
            } else {
                None
            }
        }
    };
}
pub(in crate::compiler::parser) use peek;

macro_rules! matches_token_order {
    ($self:ident $(, $token_pattern:tt)*) => {
        (|| {
            let i = 0;
            let _start_line = current!($self, line);
            loop {
                matches_token_order!(@$self, i, _start_line $(,$token_pattern)*);
                break;
            }
            return true;
        })()
    };
    (@$self:ident, $i:ident, $start_line:ident) => {
        // No more tokens
    };

    (@$self:ident, $i:ident, $start_line:ident, Type) => {
        if !$self.check_if_type($i) {
            return false;
        }
    };
    (@$self:ident, $i:ident, $start_line:ident, Type $(, $rest:tt)*) => {
        matches_token_order!(@$self, $i, $start_line, Type);
        let i = $i + 1;
        matches_token_order!(@$self, i, $start_line $(, $rest)*);
    };
    (@$self:ident, $i:ident, $start_line:ident, ..., $next_token:ident) => {
        {
            let mut i = $i;
            loop {
                if let Some((ttype, metadata)) = peek!($self, i, ttype, metadata) {
                    if ttype.is(&$next_token) {
                        break;
                    }
                    if metadata.get_line() > $start_line {
                        return false;
                    }
                } else {
                    return false;
                }
                i += 1;
            }
            i
        }
    };
    (@$self:ident, $i:ident, $start_line:ident, ..., $next_token:ident $(, $rest:tt)*) => {
        let mut i = matches_token_order!(@$self, $i, $start_line, ..., $next_token);
        i += 1;
        matches_token_order!(@$self, i, $start_line, $(, $rest)*);
    };
    (@$self:ident, $i:ident, $start_line:ident, $token:ident) => {
        if peek!($self, $i, ttype).filter(|ttype| ttype.is(&$token)).iter().count() == 0  {
            return false;
        }
    };
    (@$self:ident, $i:ident, $start_line:ident, $token:ident $(, $rest:tt)*) => {
        matches_token_order!(@$self, $i, $start_line, $token);
        let i = $i + 1;
        matches_token_order!(@$self, i, $start_line $(, $rest)*)
    };
}
pub(in crate::compiler::parser) use matches_token_order;
