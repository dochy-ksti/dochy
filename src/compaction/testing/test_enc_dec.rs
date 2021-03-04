use crate::compaction::kval_enum::{KVal, Decimal};
use crate::compaction::{encode, decode};
use anyhow::Result;
use crate::compaction::enc_dec::encode_to_vec::encode_to_vec;


#[test]
fn test2() -> Result<()>{
    fn decimal(int : i128, dot : u8) -> KVal {
        KVal::Decimal(Decimal::new(int, dot))
    }
    let vec : Vec<KVal> = vec![
        KVal::Null,
        KVal::Bit(false),
        KVal::Bool(true),
        KVal::Byte(0),
        KVal::Byte(20),
        KVal::Byte(-128),
        KVal::Str16("amenbou".to_string()),
        KVal::Str16("".to_string()),
        KVal::Str16("012345678901234".to_string()),
        KVal::Int(0),
        KVal::Int(-50),
        KVal::Int(200),
        KVal::Int(-200),
        KVal::Int(32767),
        KVal::Int(-32767),
        KVal::Int(-32768),
        KVal::Int(32768),
        KVal::Float(0.1f32),
        KVal::Str256("01234567890123456".to_string()),
        KVal::Str256("".to_string()),
        KVal::Str256("01234".to_string()),
        KVal::Double(0.1f64),
        decimal(0, 0),
        decimal(0, 4),
        decimal(65536*65536*65536, 4),
        decimal(-65536*65536*65536, 4),
        decimal(65536*65536*65536*65536, 0),
        decimal(-65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536 / 4, 0),
        decimal(65536*65536*65536*65536*65536, 0),

        KVal::BigStr(String::from_utf8(vec!['a' as u8; 256]).unwrap()),
        KVal::BigStr("".to_string()),
        KVal::Undefined(0),
        KVal::Undefined(7),
    ];

    let encoded = encode_to_vec(&vec)?;
    let (decoded, size) = decode(&mut encoded.as_slice())?;
    assert_eq!(encoded.len(), size);
    assert_eq!(vec, decoded);
    Ok(())
}