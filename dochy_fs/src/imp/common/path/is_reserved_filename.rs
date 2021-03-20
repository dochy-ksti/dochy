use crate::imp::common::archive::archive_default_name::ARCHIVE_DEFAULT_NAME;
use crate::imp::common::path::created_time_file::CREATED_TIME_FILE_NAME;

pub(crate) fn is_reserved_filename(filename : &str) -> bool{
    filename == ARCHIVE_DEFAULT_NAME || filename == CREATED_TIME_FILE_NAME
}