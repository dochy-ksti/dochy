


pub mod error;

#[allow(dead_code)]
#[cfg(test)]
mod test_diff_history;
#[allow(dead_code)]
#[cfg(test)]
mod test_fs;
#[allow(dead_code)]
#[cfg(test)]
mod test_simple_history;
//#[allow(dead_code)]
mod imp;

pub mod common{
    pub use crate::fs::imp::common::current_src::CurrentSrc;

    pub use crate::fs::imp::common::path::hash_dir_path::hash_dir_path;
    pub use crate::fs::imp::common::path::get_hash_times::get_hash_times;
    pub use crate::fs::imp::common::remove_hash_dir::remove_hash_dir;

    pub use crate::fs::imp::common::archive::archive_opt::JSON_ARC_OPT;
    pub use crate::fs::imp::common::archive::archive_default_name::ARCHIVE_DEFAULT_NAME;

    pub use crate::fs::imp::common::archive::read_archive::read_archive;
    pub use crate::fs::imp::common::archive::load_archive::load_archive;

    pub use crate::fs::imp::common::list::file_data::FileData;

    pub mod hash{
        pub use crate::fs::imp::common::path::hash::folder_name_to_hash;
        pub use crate::fs::imp::common::path::hash::hash_to_folder_name;
    }
}

pub mod history{
    pub use crate::fs::imp::history::algo::history_options::HistoryOptions;
    pub use crate::fs::imp::history::algo::history_options::CumulativeOptions;
    pub use crate::fs::imp::history::algo::history_options::HistoryOptionsBuilder;
    pub use crate::fs::imp::history::algo::history_options::CumulativeOptionsBuilder;

    pub use crate::fs::imp::history::diff_and_cache::dochy_cache::DochyCache;

    pub use crate::fs::imp::history::file_hist::file_history::FileHistory;
    pub use crate::fs::imp::history::remove::history_remover::HistoryRemover;

    pub use crate::fs::imp::history::file_hist::file_histories::FileHistories;

    pub use crate::fs::imp::history::file_hist::history_file_data::HistoryFileData;

    pub use crate::fs::imp::history::file_name::calc_filename::calc_filename;
    pub use crate::fs::imp::history::file_name::file_name_props::FileNameProps;

    pub use crate::fs::imp::history::pub_fn::next::next;
    pub use crate::fs::imp::history::pub_fn::start_new::start_new;

    pub use crate::fs::imp::history::pub_fn::list_histories::list_histories;
    pub use crate::fs::imp::history::pub_fn::load_history_file::load_history_file;
    pub use crate::fs::imp::history::pub_fn::load_history_file::load_history_file_data;
}

pub mod filesys{
    pub use crate::fs::imp::filesys::save_file::save_file;
    pub use crate::fs::imp::filesys::list_saved_files::list_saved_files;
    pub use crate::fs::imp::filesys::load_saved_file::load_saved_file;
    pub use crate::fs::imp::filesys::remove_saved_file::remove_saved_file;
}