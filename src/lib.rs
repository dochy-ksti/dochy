#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;

#[cfg(test)]
mod sample_test;

#[allow(dead_code)]
#[cfg(test)]
mod testing;

pub mod intf;
pub mod core;
pub mod diff;
pub mod fs;
pub mod error;


//TODO: 多態性の実装をちゃんとできるようにする。多分coreへの機能追加が必要
//TODO: 多態性の説明を完成させる

//TODO: 全部documentationする
//TODO: Redditに出して反応を見る このペースだと何ヶ月かかるんじゃあ・・・
