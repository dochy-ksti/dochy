use dochy::error::DpResult;
use dochy::fs::common::{CurrentSrc};
use std::path::{Path};


use dochy::fs::history::{HistoryInfo, list_histories, load_history_file, save_history_file_nb, save_history_file_nb_if_vacant};
use std::sync::atomic::Ordering;
use std::time::Duration;


#[test]
fn test_atomic() -> DpResult<()> {
    let root_dir = Path::new("src/fs_test/test_atomic");
    let history_dir = root_dir.join("history_dir");
    let src_dir = root_dir.join("src_dir");

    std::fs::remove_dir_all(&history_dir).ok();
    std::fs::create_dir(&history_dir).ok();


    let info = HistoryInfo::create(&history_dir,
                                   CurrentSrc::from_src_dir(&src_dir), ()).unwrap();


    rayon::scope(|s| {
        for i in 0..10000 {
            s.spawn(|a| {
                info.peekable().queued_atomic().fetch_add(1, Ordering::Relaxed);
                info.peekable().queued_atomic().fetch_sub(1, Ordering::Relaxed);
                info.peekable().queued_atomic().fetch_add(1, Ordering::Relaxed);
            });
        }
    });


    std::thread::sleep(Duration::from_millis(100));
    dbg!("queued {}", info.peekable().queued());
    // if info.peekable().queued() == 0 {
    //     break;
    // }


    Ok(())
}