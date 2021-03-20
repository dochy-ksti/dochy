use crate::error::FsResult;
use std::env::temp_dir;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use crate::imp::history::fs::next::next;
use rand::Rng;
use crate::test_simple_history::simple_diff::sd_cache::SdCache;
use crate::imp::history::algo::history_options::{HistoryOptions, HistoryOptionsBuilder};
use crate::imp::history::fs::load::load;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::history::CumulativeOptionsBuilder;

//#[test]
fn test_max_phase0_with_cumu() -> FsResult<()> {
    let dir = temp_dir();
    let mut rng = rand::thread_rng();
    let dir_name = format!("test_simple_diff_{}",rng.gen_range(0..100_000_000));
    let dir = dir.join(&dir_name);
    //std::filesys::remove_dir(&dir).ok();
    std::fs::create_dir(&dir).ok();

    let op = HistoryOptions::from(
        HistoryOptionsBuilder {
            max_phase: 0,
            update_phase_a : true,
            cumulative: Some(CumulativeOptionsBuilder{
                limit_count : None,
                limit_nth : None,
            }),
        })?;

    let mut data : SdData = SdData::new();
    let mut cache = SdCache::new();
    let repeat = 100;
    for _rep in 0..repeat{
        let n = rng.gen_range(1..=3);

        for _ in 0..n {
            data.mutate_randomly();
        }

        next(None, &data, &mut cache,&dir, &op)?;
        let history = create_file_history(&dir, Some(op.max_phase()))?;
        let loaded = load(&history.newest_file_path(&dir)?, &history, &mut cache, &op)?;
        assert_eq!(loaded, data)
    }

    // let hoge = show_dir_contents(&dir)?;
    // for (name,size) in &hoge{
    //     println!("{} {}", name, size);
    // }

    Ok(())
}