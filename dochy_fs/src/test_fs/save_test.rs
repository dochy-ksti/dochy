use crate::imp::filesys::save_dochy_file::save_dochy_file;
use dochy_core::intf::RootObjectPtr;
use dochy_core::intf::root::{set_bool, get_bool, set_int, get_int};
use dochy_core::structs::Qv;
use tempfile::tempdir;
use crate::error::FsResult;
use crate::imp::filesys::load_saved_file::load_saved_file;
use crate::test_fs::copy_dir_all::copy_dir_all;
use crate::imp::common::current_src::CurrentSrc;
use crate::imp::common::list::list_files::list_files;

//#[test]
fn save_test() -> FsResult<()> {
    let dir = tempdir()?;
    let proj_dir_path = dir.path();
    let src_dir_path = proj_dir_path.join("simple_src");

    copy_dir_all("src/json_dir/simple", &src_dir_path)?;

    let first_save_path;
    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let (src_root, hash) = current_src.create_root_and_hash(false)?;
        let mut root = src_root.clone();
        let p = RootObjectPtr::new(&mut root);
        set_bool(p, "b", Qv::Val(true));

        let path = save_dochy_file(proj_dir_path, "test1", &root, &current_src, hash, &src_root, false)?;

        let mut loaded = load_saved_file(&path, hash, &src_root,false)?;
        first_save_path = path;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);
    }
    std::fs::copy("src/json_dir/simple_mod1/root.json5", &src_dir_path.join("root.json5"))?;

    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let (src_root, hash) = current_src.create_root_and_hash(false)?;
        let mut root = src_root.clone();
        let p = RootObjectPtr::new(&mut root);
        set_int(p, "int", Qv::Val(-1));

        let path = save_dochy_file(proj_dir_path, "test2", &root, &current_src, hash, &src_root, false)?;
        let mut loaded = load_saved_file(&path, hash, &src_root, false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &-1);
    }

    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let (src_root, hash) = current_src.create_root_and_hash(false)?;
        let mut loaded = load_saved_file(&first_save_path, hash, &src_root, false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);

        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &10);

        let a = get_int(p, "added")?;
        assert_eq!(a.value().unwrap(), &100);
    }

    for _file in list_files(proj_dir_path)?{
    }


    Ok(())
}