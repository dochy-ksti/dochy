use crate::fs::imp::history::file_name::file_name_props::FileNameProps;
use crate::fs::imp::history::file_hist::file_history::FileHistory;
use std::path::{PathBuf, Path};
use crate::fs::error::FsResult;

pub(crate) struct Ancestors{
    vec : Vec<FileNameProps>,
}

impl Ancestors {
    //pub(crate) fn empty(dir_path : PathBuf) -> Ancestors{ Ancestors{ vec : vec![] }}

    pub(crate) fn create(history: &FileHistory,
                         props: &FileNameProps,
                         max_phase: usize,
                         cumulative : bool) -> FsResult<Ancestors> {
        Ok(Ancestors{ vec : create_ancestors(history, props, max_phase, cumulative)? })
    }

    pub(crate) fn calc_paths(&self, history_dir: &Path) -> Vec<PathBuf>{
        self.vec.iter().map(|props|{
           history_dir.join(props.calc_filename())
        }).collect()
    }
}


fn create_ancestors(history: &FileHistory,
                    props: &FileNameProps,
                    max_phase: usize,
                    cumulative : bool) -> FsResult<Vec<FileNameProps>>{
    let mut vec : Vec<FileNameProps> = vec![];

    let first_his = history.get_ctl(props.control())?;
    let mut his = first_his;
    let len = props.order().len();
    if len == 0{
        //nextのorderが空の時点で不正であるが、ここでは関知しない
        return Ok(vec);
    }

    for (i, order) in props.order().iter().take(len-1).enumerate(){
        vec.push(his.items().get(order)?.clone());
        if let Some(next_his) = his.children().get(order) {
            his = next_his;
        } else{
            //cumulativeでない場合、ancestorはorderの最後の一つ前までである。
            //orderの最後のファイルはこれから作るので当然まだ存在しないし、
            //そのphaseがまだない場合childrenも存在していなくて正常
            if i == len - 2 && props.order_last() == 0{
                //別に場合分けしなくても、historyが不正でない限り結果は変わらないが・・・
                return Ok(vec);
            } else{
                //ここに来るのは俺のバグなのでpanicでも良い・・・
                Err(format!("Invalid history, order {:?} his {:?}", props.order(), first_his))?
            }
        }
    }

    if len - 1 == max_phase && cumulative{
        let order_last = props.order_last();
        for i in 0..order_last{
            vec.push(his.items().get(&i)?.clone())
        }
    }
    return Ok(vec)
}