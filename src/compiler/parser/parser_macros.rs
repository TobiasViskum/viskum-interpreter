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
