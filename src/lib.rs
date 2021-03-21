#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;

#[cfg(test)]
mod sample_test;

pub mod json5;
pub mod compaction;
pub mod intf;
pub mod archiver;
pub mod core;
pub mod diff;
pub mod fs;


//TODO: coreに新配列 Binaryを追加
//TODO: IntArrayとFloatArrayのエンコードをcompaction::Binaryにする
//TODO: intf に obj::hoge_default_value() を追加
//TODO: hoge_default_valueを使ってVersion Awarenessの説明を完成させる

//TODO: 多態性の実装をちゃんとできるようにする。多分coreへの機能追加が必要
//TODO: 多態性の説明を完成させる

//TODO: 全部documentationする
//TODO: Redditに出して反応を見る このペースだと何ヶ月かかるんじゃあ・・・
