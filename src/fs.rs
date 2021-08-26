pub mod common{
    pub use dochy_fs::common::CurrentSrc;

    pub use dochy_fs::common::hash_dir_path;
    pub use dochy_fs::common::hash_dir_paths;
    pub use dochy_fs::common::get_hash_times;
    pub use dochy_fs::common::remove_hash_dir;

    pub use dochy_fs::common::JSON_ARC_OPT;
    pub use dochy_fs::common::reserved_filename;

    pub use dochy_fs::common::FileData;

    //pub use dochy_fs::common::read_archive;
    //pub use dochy_fs::common::load_archive;


    // pub mod hash{
    //     pub use dochy_fs::common::hash::folder_name_to_hash;
    //     pub use dochy_fs::common::hash::hash_to_folder_name;
    // }
}

pub mod history{
    pub use dochy_fs::history::HistoryOptions;
    pub use dochy_fs::history::CumulativeOptions;
    pub use dochy_fs::history::HistoryOptionsBuilder;
    pub use dochy_fs::history::CumulativeOptionsBuilder;

    pub use dochy_fs::history::FileHistory;
    pub use dochy_fs::history::HistoryRemover;

    pub use dochy_fs::history::FileHistories;

    pub use dochy_fs::history::HistoryFileData;

    pub use dochy_fs::history::calc_filename;
    pub use dochy_fs::history::FileNameProps;

    pub use dochy_fs::history::save_history_file;
    pub use dochy_fs::history::save_history_file_nb;
    pub use dochy_fs::history::save_history_file_nb_if_vacant;

    pub use dochy_fs::history::list_histories;
    pub use dochy_fs::history::load_history_file;
    pub use dochy_fs::history::load_history_file_data;

    pub use dochy_fs::history::set_current_root_obj_info;
    pub use dochy_fs::history::get_peekable_info;
    pub use dochy_fs::history::init_dochy_cache_us;
    pub use dochy_fs::history::PeekableCacheInfo;
    pub use dochy_fs::history::CurrentRootObjInfo;

    pub use dochy_fs::history::HistoryInfo;
}

pub mod filesys{
    pub use dochy_fs::filesys::save_dochy_file;
    pub use dochy_fs::filesys::save_dochy_file_nb;
    pub use dochy_fs::filesys::list_dochy_files;
    pub use dochy_fs::filesys::load_dochy_file;
    pub use dochy_fs::filesys::remove_dochy_file;
    pub use dochy_fs::filesys::force_update_and_get_info_us;
    pub use dochy_fs::filesys::SaveDirInfo;

}

pub mod error{
    pub use dochy_fs::error::FsResult;
    pub use dochy_fs::error::FsError;
}

