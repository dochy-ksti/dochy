#![feature(try_trait)]
#![feature(test)]
extern crate test;


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
    pub use crate::imp::common::current_src::CurrentSrc;

    pub use crate::imp::common::path::hash_dir_path::hash_dir_path;
    pub use crate::imp::common::path::hash_dir_path::hash_dir_paths;
    pub use crate::imp::common::path::get_hash_times::get_hash_times;
    pub use crate::imp::common::remove_hash_dir::remove_hash_dir;

    pub use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
    pub use crate::imp::common::path::reserved_filename;

    pub use crate::imp::common::archive::read_archive::read_archive;
    pub use crate::imp::common::archive::load_archive::load_archive;

    pub use crate::imp::common::list::file_data::FileData;

    pub use crate::imp::common::dochy_cache::DochyCache;

    pub mod hash{
        pub use crate::imp::common::path::hash::folder_name_to_hash;
        pub use crate::imp::common::path::hash::hash_to_folder_name;
    }
}

pub mod history{
    pub use crate::imp::history::algo::history_options::HistoryOptions;
    pub use crate::imp::history::algo::history_options::CumulativeOptions;
    pub use crate::imp::history::algo::history_options::HistoryOptionsBuilder;
    pub use crate::imp::history::algo::history_options::CumulativeOptionsBuilder;

    pub use crate::imp::history::file_hist::file_history::FileHistory;
    pub use crate::imp::history::remove::history_remover::HistoryRemover;

    pub use crate::imp::history::file_hist::file_histories::FileHistories;

    pub use crate::imp::history::file_hist::history_file_data::HistoryFileData;

    pub use crate::imp::history::file_name::calc_filename::calc_filename;
    pub use crate::imp::history::file_name::file_name_props::FileNameProps;

    pub use crate::imp::history::pub_fn::save_history_file::save_history_file;
    pub use crate::imp::history::pub_fn::save_history_file::save_history_file_async;

    pub use crate::imp::history::pub_fn::list_histories::list_histories;
    pub use crate::imp::history::pub_fn::load_history_file::load_history_file;
    pub use crate::imp::history::pub_fn::load_history_file::load_history_file_data;

    pub use crate::imp::history::current_root_obj_info::history_cache_map::set_current_root_obj_info;
    pub use crate::imp::history::current_root_obj_info::history_cache_map::get_peekable_info;
    pub use crate::imp::history::current_root_obj_info::history_cache_map::init_dochy_cache_us;
    pub use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
    pub use crate::imp::history::current_root_obj_info::history_cache_item::PeekableCacheInfo;
}

pub mod filesys{
    pub use crate::imp::filesys::save_file::save_file;
    pub use crate::imp::filesys::list_saved_files::list_saved_files;
    pub use crate::imp::filesys::load_saved_file::load_saved_file;
    pub use crate::imp::filesys::remove_saved_file::remove_saved_file;
}

