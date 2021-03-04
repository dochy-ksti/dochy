use crate::core::intf::*;
use crate::core::structs::RootObject;
use crate::intf::imp::structs::root_source::RootSource;
use crate::intf::imp::generate_root_source::generate_root_source;


/// データに静的にアクセスできるラッパーを生成する。
/// RootIntf::newしそこを起点にしてアクセスする。
/// RootIntfから取れたポインタは、RootIntfが削除されれば全て不正になる
pub fn generate_interface(root : &mut RootObject) -> RootSource{
    let mem_descs = member_desc::get_member_desc(root);
    generate_root_source(&mem_descs)

}
