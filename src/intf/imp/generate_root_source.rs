use crate::core::intf::member_desc::MemberDesc;
use crate::intf::imp::structs::root_source::RootSource;
use crate::intf::imp::to_member_source::{to_member_source, MemberSource};

pub(crate ) fn generate_root_source(mems : &[MemberDesc]) -> RootSource{
    let mut members : Vec<MemberSource> = vec![];
    for mem in mems{
        members.push(to_member_source(mem))
    }
    RootSource::new(members)
}