pub mod common{
    pub use dochy_fs::common::CurrentSrc;

    pub use dochy_fs::common::hash_dir_path;
    pub use dochy_fs::common::hash_dir_paths;
    pub use dochy_fs::common::get_hash_times;
    pub use dochy_fs::common::remove_hash_dir;

    pub use dochy_fs::common::JSON_ARC_OPT;
    pub use dochy_fs::common::reserved_filename;

    //pub use dochy_fs::common::read_archive;
    //pub use dochy_fs::common::load_archive;

    pub use dochy_fs::common::FileData;

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

    pub use dochy_fs::history::DochyCache;

    pub use dochy_fs::history::FileHistory;
    pub use dochy_fs::history::HistoryRemover;

    pub use dochy_fs::history::FileHistories;

    pub use dochy_fs::history::HistoryFileData;

    pub use dochy_fs::history::calc_filename;
    pub use dochy_fs::history::FileNameProps;

    pub use dochy_fs::history::save_history_file;
    //pub use dochy_fs::history::start_new;

    pub use dochy_fs::history::list_histories;
    pub use dochy_fs::history::load_history_file;
    pub use dochy_fs::history::load_history_file_data;
}

pub mod filesys{
    pub use dochy_fs::filesys::save_file;
    pub use dochy_fs::filesys::list_saved_files;
    pub use dochy_fs::filesys::load_saved_file;
    pub use dochy_fs::filesys::remove_saved_file;
}

pub mod error{
    pub use dochy_fs::error::FsResult;
    pub use dochy_fs::error::FsError;
}

