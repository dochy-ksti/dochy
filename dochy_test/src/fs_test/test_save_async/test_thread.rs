use dochy::error::DpResult;
use dochy::fs::common::{CurrentSrc};
use std::path::{Path, PathBuf};
use dochy::core::structs::RootObject;
use rand::Rng;
use dochy::fs::filesys::{SaveDirInfo, save_dochy_file_async, load_dochy_file, list_dochy_files};
use crate::fs_test::test_save_async::test_save_async_accessor::RootIntf;
use std::time::Duration;
//use std::lazy::Lazy;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::io::Write;

static vec_lazy : Lazy<Mutex<Vec<String>>> = Lazy::new(||{
    Mutex::new(Vec::new())
});

static vec_lazy2 : Lazy<Mutex<Vec<String>>> = Lazy::new(||{
    Mutex::new(Vec::new())
});

/// FileのWriteは全くアトランダムに実行される
//#[test]
fn test_save_async() -> DpResult<()> {

    let max = 10;


    for i in 0..max{
        std::thread::spawn(move||{
            {
                let mut v = vec_lazy.lock().unwrap();

            {
                let mut f = std::fs::File::create("src/fs_test/test_save_async/test_thread_file.dat").unwrap();
                f.write(&vec![10, 20, 30]);
            }
                v.push(format!("{}", i));
            }

            let mut v = vec_lazy2.lock().unwrap();
            v.push(format!("{}",i));
        });
    }

    std::thread::sleep(Duration::from_millis(100));

    let mut v = vec_lazy.lock().unwrap();
    let hoge : &Vec<String> = &v;
    println!("{:?}", hoge);

    let mut v = vec_lazy2.lock().unwrap();
    let hoge : &Vec<String> = &v;
    println!("{:?}", hoge);

    //list_dochy_files()
    //load_dochy_file()
    Ok(())
}