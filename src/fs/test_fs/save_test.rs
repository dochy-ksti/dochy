use crate::core::json_dir_to_rust;
use crate::fs::imp::filesys::save_file::save_file;
use crate::core::intf::RootObjectPtr;
use crate::core::intf::root::{set_bool, get_bool, set_int, get_int};
use crate::core::structs::Qv;
use tempfile::tempdir;
use crate::fs::error::FsResult;
use crate::fs::imp::filesys::load_saved_file::load_saved_file;
use crate::fs::test_fs::copy_dir_all::copy_dir_all;
use crate::fs::imp::common::current_src::CurrentSrc;
use crate::fs::imp::common::list::list_files::list_files;

//#[test]
fn save_test() -> FsResult<()> {
    let dir = tempdir()?;
    let proj_dir_path = dir.path();
    let src_dir_path = proj_dir_path.join("simple_src");
    let current_src = CurrentSrc::SrcDir(src_dir_path.clone());

    copy_dir_all("src/json_dir/simple", &src_dir_path)?;

    let first_save_path;
    {
        let mut root = json_dir_to_rust(&src_dir_path, false)?;
        let p = RootObjectPtr::new(&mut root);
        set_bool(p, "b", Qv::Val(true));

        let path = save_file(proj_dir_path,&mut root, &current_src,  "test1", false)?;

        let mut loaded = load_saved_file(&path, &current_src, false)?;
        first_save_path = path;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);
    }
    std::fs::copy("src/json_dir/simple_mod1/root.json5", &src_dir_path.join("root.json5"))?;

    {
        let mut root = json_dir_to_rust(&src_dir_path, false)?;
        let p = RootObjectPtr::new(&mut root);
        set_int(p, "int", Qv::Val(-1));

        let path = save_file(proj_dir_path,&mut root, &CurrentSrc::SrcDir(src_dir_path.clone()), "test2", false)?;
        let mut loaded = load_saved_file(&path, &current_src, false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &-1);
    }

    {
        let mut loaded = load_saved_file(&first_save_path, &CurrentSrc::SrcDir(src_dir_path.clone()), false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);

        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &10);

        let a = get_int(p, "added")?;
        assert_eq!(a.value().unwrap(), &100);
    }

    for file in list_files(proj_dir_path)?{
        dbg!(file);
    }


    dbg!("ok");
    Ok(())
}