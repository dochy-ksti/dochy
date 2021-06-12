#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;



pub mod intf;
pub mod core;
pub mod fs;
pub mod error;


//TODO: セーブ、ロード時に刻印を残す
//TODO: ロード後のファイルから派生できるようにする
//TODO: 途中での派生を考慮した依存関係、削除ルーチンを作る
//TODO: キャッシュは全段階で行うようにする。フェーズ数は3を基本に。フェーズ変更オプションが必要かどうかはわからないが一旦消してしまってもいい
//TODO: BinaryはRcで持つことにして、クローン時にクローンされないようにする
//TODO: ファイルは一旦書き出してからリネームするようにする
//TODO: フォルダ削除時に途中で強制終了があると中途半端なファイルだけ残り不正な状態になる可能性があるので、まずフォルダ名を変更しフォルダ内のファイルを無効にしてから削除するようにする
//TODO: RootのMutListも内部でDefと実Listに分け、Defを完全にimmutableにした上でRcにくるむ

//TODO: 全部documentationする
//TODO: Redditに出して反応を見る このペースだと何ヶ月かかるんじゃあ・・・
