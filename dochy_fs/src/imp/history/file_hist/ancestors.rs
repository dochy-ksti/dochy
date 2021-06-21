use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history::FileHistory;
use std::path::{PathBuf, Path};
use crate::error::FsResult;

// pub(crate) struct Ancestors{
//     vec : Vec<FileNameProps>,
// }
//
// impl Ancestors {
//     //pub(crate) fn empty(dir_path : PathBuf) -> Ancestors{ Ancestors{ vec : vec![] }}
//
//     pub(crate) fn create(history: &FileHistory,
//                          props: &FileNameProps,
//                          max_phase: usize,
//                          cumulative : bool) -> FsResult<Ancestors> {
//         let mut ancestors_rev = create_ancestors_rev(history, props, max_phase, cumulative)?;
//         ancestors_rev.reverse();
//         Ok(Ancestors{ vec : ancestors })
//     }
//
//     pub(crate) fn calc_paths(&self, history_dir: &Path) -> Vec<PathBuf>{
//         self.vec.iter().map(|props|{
//            history_dir.join(props.calc_filename())
//         }).collect()
//     }
// }

pub(crate) fn create_ancestors_paths<'a>(history: &'a FileHistory,
                                        props: &'a FileNameProps,
                                        max_phase: usize,
                                        cumulative : bool,
                                         history_dir : &Path) -> FsResult<Vec<PathBuf>>{
    let mut rev = create_ancestors_rev(history, props, max_phase, cumulative)?;
    rev.reverse();
    Ok(calc_ancestors_paths(&rev, history_dir))
}

pub(crate) fn calc_ancestors_paths(ancestors : &[&FileNameProps], history_dir : &Path) -> Vec<PathBuf>{
    ancestors.iter().map(|&props| history_dir.join(props.calc_filename())).collect()
}

///直近の親からスタートして、最も遠い祖先まで並べる。なので通常の親子関係の逆順になる
pub(crate) fn create_ancestors_rev<'a>(history: &'a FileHistory,
                    props: &'a FileNameProps,
                    max_phase: usize,
                    cumulative : bool) -> FsResult<Vec<&'a FileNameProps>>{
    let mut vec : Vec<&FileNameProps> = vec![];

    let len = props.order().len();
    if len == 0{
        return Ok(vec);
    }
    let mut props = props;

    if len - 1 == max_phase && cumulative {
        let order_last = props.order_last();
        let order_base = &props.order()[0..len - 1];
        let parent = history.get_item(props.prev_ctl(), order_base)?;
        for inv_i in 0..order_last {
            let ind = order_last - 1 - inv_i;
            vec.push(parent.items().get(&ind)?)
        }

        props = history.get_props(props.prev_ctl(), order_base)?;
    }

    loop{
        let prev_ctl = props.prev_ctl();
        let len = props.order().len()-1;
        if 1 <= len {
            let order = &props.order()[0..len];
            if let Some(p) = history.get_props(prev_ctl, order){
                vec.push(p);
                props = p;
            }
        } else{
            return Ok(vec)
        }
    }

}
//
// fn create_ancestors_b(history: &FileHistory,
//                     props: &FileNameProps,
//                     max_phase: usize,
//                     cumulative : bool) -> FsResult<Vec<FileNameProps>>{
//     let mut vec : Vec<FileNameProps> = vec![];
//
//     let first_his = history.get_ctl(props.control())?;
//     let mut his = first_his;
//     let len = props.order().len();
//     if len == 0{
//         //nextのorderが空の時点で不正であるが、ここでは関知しない
//         return Ok(vec);
//     }
//
//     kokowoyare
//
//     for (i, order) in props.order().iter().take(len-1).enumerate(){
//         vec.push(his.items().get(order)?.clone());
//         if let Some(next_his) = his.children().get(order) {
//             his = next_his;
//         } else{
//             //cumulativeでない場合、ancestorはorderの最後の一つ前までである。
//             //orderの最後のファイルはこれから作るので当然まだ存在しないし、
//             //そのphaseがまだない場合childrenも存在していなくて正常
//             if i == len - 2 && props.order_last() == 0{
//                 //別に場合分けしなくても、historyが不正でない限り結果は変わらないが・・・
//                 return Ok(vec);
//             } else{
//                 //ここに来るのは俺のバグなのでpanicでも良い・・・
//                 Err(format!("Invalid history, order {:?} his {:?}", props.order(), first_his))?
//             }
//         }
//     }
//
//     if len - 1 == max_phase && cumulative{
//         let order_last = props.order_last();
//         for i in 0..order_last{
//             vec.push(his.items().get(&i)?.clone())
//         }
//     }
//     return Ok(vec)
// }