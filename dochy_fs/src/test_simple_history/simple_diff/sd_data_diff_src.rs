use crate::imp::history::diff_and_cache::diff_src::{DiffSrc};
use crate::test_simple_history::simple_diff::sd_data::SdData;
use crate::error::FsResult;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use crate::test_simple_history::simple_diff::create_sd_diff::create_sd_diff;

impl DiffSrc<SdDiff> for SdData{

    fn create_diff(&self, from: &Self) -> FsResult<SdDiff> {
        Ok(create_sd_diff(from, self))
    }

    fn apply_diff(&mut self, diff: &SdDiff) -> FsResult<()> {
        SdData::apply_diff(self, diff)
    }



    // fn get_diff<W: Write>(&self, ancestors : &Ancestors, write: &mut W) -> FsResult<()>{
    //     let mut base = SdData::new();
    //     for data in ancestors.iterate_file_data(){
    //         let data = data?;
    //         let diff = SdDiff::decode_kvals(&data)?;
    //         base.apply_diff(&diff)?;
    //     }
    //     let diff = create_diff(&base, self);
    //     diff.encode(write)?;
    //     Ok(())
    // }

}