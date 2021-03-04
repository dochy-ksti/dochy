use crate::fs::imp::history::file_name::calc_filename::calc_filename;
use crate::fs::error::FsResult;
use crate::fs::imp::history::file_name::analyze_file_name::analyze_file_name;

///Metadata history files' filename contains.
///The filename can be precisely restored from it.
#[derive(Debug, Clone)]
pub struct FileNameProps {
    control: u32,
    order : Vec<u32>,
    tag: Option<String>,
}

impl FileNameProps{

    /// Create item from metadata.
    pub fn new(control : u32, order : Vec<u32>, tag : Option<String>) -> FsResult<FileNameProps>{
        if order.len() == 0{
            Err("order.len() must be >= 1")?
        }
        Ok(FileNameProps{ control, order, tag })
    }

    pub fn from(filename : &str) -> FsResult<FileNameProps>{
        Ok(analyze_file_name(filename, None)?)
    }

    ///how many times "start_new" is called in this directory when this item is created.
    pub fn control(&self) -> u32{ self.control }

    /// numbers which describes how this item is derived.
    /// if the order is `[3,8,2]`, this item depends on items which have orders of `[3]` and `[3,8]`
    ///
    /// order.len >= 1
    pub fn order(&self) -> &[u32]{ &self.order }
    pub(crate) fn order_last(&self) -> u32{ *self.order.last().unwrap() }

    /// string data users can append in this item's filename.
    pub fn tag(&self) -> Option<&str>{ self.tag.as_ref().map(|s| s.as_str()) }

    /// calculate the filename from the metadata this item contains.
    pub fn calc_filename(&self) -> String{
        calc_filename(self.tag(), self.control(), self.order())
    }

    // ///dir_path.join(filename)
    // pub fn calc_file_path<P:AsRef<Path>>(&self, history_hash_dir: P) -> PathBuf{
    //     history_hash_dir.as_ref().join(self.calc_filename())
    // }

    pub(crate) fn create_next_phase_props(&self, tag : Option<String>, next_phase : usize) -> Option<FileNameProps> {
        //dbg!(next_phase);
        let n = self.control();
        let mut order : Vec<u32> = self.order()[0..next_phase].iter().map(|i| *i).collect();
        let len = self.order().len();
        //dbg!(len);
        if next_phase == len{
            order.push(0);
        } else if next_phase < len{
            order.push(self.order[next_phase] + 1);
        } else{
            return None;
        }
        return Some(FileNameProps::new(n, order, tag).ok()?)
    }

}