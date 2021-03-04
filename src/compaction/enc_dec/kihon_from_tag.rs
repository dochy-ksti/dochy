#[derive(Debug, PartialEq)]
pub(crate) enum KihonFromTag{
    Null,
    Bit(bool),
    Bool(bool),
    Byte,
    Str16(u8),
    Int(u8),
    Float,
    Str256,
    Double,
    Decimal(u8),
    BigStr(u8),
    Undefined(u8),
}