use crate::core::error::CoreResult;
use crate::core::imp::json_to_rust::validation::validate_data::validate_table;
use crate::core::imp::json_to_rust::names::Names;
use crate::core::imp::json_to_rust::validation::validate_list::validate_const_list;
use crate::core::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;
use crate::core::imp::json_to_rust::validation::validate_list_def::validate_list_def;
use crate::core::imp::structs::root_obj::RootObject;
use crate::core::imp::structs::root_value::RootValue;
use crate::core::imp::json_to_rust::validation::validate_old_def_mem::validate_old_root_def_mem;

/// json読み出し時のチェックがあり、adjust時のチェックもある。
/// それらでは補足しきれないチェックをするのがこれの役割。
///
/// ここではどこまでチェックするのか、補足しきれないものというのはどこまでか、というのが問題なのだけど、基本的には全部チェックする。他のチェックに依存しない。
/// というところを目指すことになるだろう。ダブルチェックや。
///
/// 型により可能な値はかなり限られるので、その中ですべてが妥当な値であるかどうかを調べることも出来ないことはないはず。
/// 基本的に
/// ・sabunがdefにあり、型があっていて、(optional)oldじゃないか
/// ・ref先が存在し、(optional)oldじゃないか
/// ・InnerListDefがListのDefにあり、InnerListがListItemにあるか。Root直下にInnerがないか。
/// ・Oldに指定されたメンバまたはIDが存在するか
/// を確かめている
///
/// can_use_oldがtrueだとoldでも気にしなくなる。Jsonで初期値を読み込んだ後はcan_use_old=false,
/// 旧バージョンから以降した場合はcan_use_old=trueでやるとよかろうと思う
pub(crate) fn validate_root(root : &RootObject, can_use_old: bool) -> CoreResult<()>{
    validate_old_root_def_mem(root.old(), root.default(), &Names::new("."))?;

    for (name, (_id, val)) in root.default(){
        let names = &Names::new(name);
        //RootはOldでも値を入れざるを得ないので入れて良い
        //なのでここではOldは無視
        match val {
            RootValue::Param(p, _) => {
                if let Some(sab) = root.sabun().get(name){
                    if p.acceptable(sab) == false{
                        Err(format!("Root's member {}'s sabun is invalid", name))?
                    }
                }
            },
            RootValue::Table(data) =>{
                validate_list_def(data.default(), root, can_use_old, false, names)?;
                validate_table(data.default(), data.list(), root, data.old(), can_use_old, names)?
            },
            RootValue::CList(list) =>{
                validate_list_def(list.default(), root, can_use_old, false, names)?;
                validate_const_list(list.default(), list.list(), root, can_use_old, names)?
            },
            RootValue::MList(m) =>{
                //validate_compatible(m.default(), m.compatible(), root, can_use_old, names)?;
                validate_list_def(m.default(), root, can_use_old, true, names)?;
                validate_mut_list(m.default(), m.list(),  root, can_use_old, names)?
            },
        }
    }

    return Ok(());
}