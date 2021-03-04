use crate::diff::imp::read::reader::Reader;
use crate::diff::diff_error::DiffError;
use crate::diff::imp::write::store_ids::StoredIDs;

pub(crate) fn read_stored_ids(r : &mut Reader) -> Result<StoredIDs, DiffError>{
    if r.read()?.as_bool()?{
        let bits = r.read()?.as_i64()? as u64;
        if bits == 0{
            return Ok(StoredIDs::Zero);
        } else{
            return Ok(StoredIDs::U64(bits));
        }
    } else{
        if r.read()?.as_bool()?{
            let vec = r.read_u64_array()?;
            return Ok(StoredIDs::Bits(vec))
        } else{
            let vec = r.read_usize_array()?;
            return Ok(StoredIDs::Numbers(vec));
        }
    }
}