use std::path::Path;
use crate::fs::error::FsResult;
//use crate::fs::imp::history::file_name::find_newest_his_file::find_newest_his_file;
use crate::fs::imp::history::fs::write_phase_a::write_phase_a;
use crate::fs::imp::history::fs::first::first;
use crate::fs::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::fs::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::fs::imp::history::diff_and_cache::cacher::Cache;
use crate::fs::imp::history::file_hist::create_file_history::create_file_history;

pub(crate) fn start_new<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>,
    diff_src : &S,
    cache : &mut C,
    history_hash_dir: &Path,
    max_phase : usize) -> FsResult<()>{

    //file history は OS にキャッシュされており、基本的にノーコストで取り出せる、と考えよう。そうしないと単純に出来ない
    let history = create_file_history(history_hash_dir, Some(max_phase))?;

    if let Some(prop) = history.get_newest_prop(){

        write_phase_a(tag, prop.control() + 1, diff_src, cache, history_hash_dir)
    } else{
        first(tag, diff_src, cache, history_hash_dir)
    }

}