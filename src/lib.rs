#![feature(try_trait)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;



pub mod intf;
pub mod core;
pub mod fs;
pub mod error;


//TODO: セーブ、ロード時にSingletonにWeak参照を記録し、セーブ時のオブジェクトが同一かどうかを確かめられるようにする。LatestFileInfo
//TODO: そのSingletonはMutexにくるみ、少なくとも同じプロセスからはhistoryに対して並列アクセス出来なくする
//TODO: 外部からLatestFileInfoを直接書き換えられるようにする。これによって、LastAutosaveFile名がわかれば、Simpleで保存したファイルをロードしたとき、LatestFileInfoを書き換えることで、Autosave時に派生ファイルを作ることができる（あれば)
//TODO: ロード後のファイルから派生できるようにする
//TODO: 途中での派生を考慮した依存関係、削除ルーチンを作る
//TODO: キャッシュは全段階で行うようにする。
//TODO: HistoryOptionsはソースに持たせる。フェーズ数は3を基本に。フェーズ変更オプションが必要かどうかはわからないが一旦消してしまっていいだろう
//TODO: BinaryはRcで持つことにして、クローン時にクローンされないようにする。Rcの同一性でBinaryの同一性を確かめるようにする
//TODO: MItemもRcでもち、コピー、比較を簡略化する。MListもRcでくるむかは要検討。
//TODO: ファイルは一旦tmpファイルに書き出してから書き出してからリネームするようにする
//TODO: フォルダ削除時に途中で強制終了があると中途半端なファイルだけ残り不正な状態になる可能性があるので、まずフォルダ名を変更しフォルダ内のファイルを無効にしてから削除するようにする
//TODO: RootのMutListも内部でDefと実Listに分け、Defを完全にimmutableにした上でRcにくるむ
//TODO: ロード処理をマルチスレッド化する(2スレッドがバランスがいいか？)
//TODO: CopySaveFileToPhase0を作る

//TODO: 全部documentationする
//TODO: Redditに出して反応を見る このペースだと何ヶ月かかるんじゃあ・・・
