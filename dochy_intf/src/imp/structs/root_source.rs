use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::to_member_source::MemberSource;
use dochy_core::structs::ParamType;

pub struct RootSource{
    members : Vec<MemberSource>,
}
impl RootSource{
    pub(crate) fn new(members : Vec<MemberSource>) -> RootSource{
        RootSource{ members }
    }
    pub(crate) fn members(&self) -> &[MemberSource]{
        &self.members
    }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        sb.push(0,     "\
use dochy::core::intf::*;
use dochy::core::structs::*;
#[derive(Debug, PartialEq)]
pub struct RootIntf{
    root : Box<RootObject>,
    ptr : RootObjectPtr,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
		let mut root = Box::new(obj);
		let ptr = RootObjectPtr::new(root.as_mut());
		RootIntf { root, ptr }
	}
    pub unsafe fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub unsafe fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
");
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    match param.param_type() {
                        ParamType::Binary => {
                            sb.push_without_newline(1, &param.get("root"));
                            sb.push_without_newline(1, &param.get_def("root"));
                            sb.push_without_newline(1, &param.get_immutable("root"));
                            sb.push_without_newline(1, &param.get_mutable("root"));
                            sb.push_without_newline(1, &param.set("root"));
                        }
                        _ => {
                            sb.push_without_newline(1, &param.get("root"));
                            sb.push_without_newline(1, &param.get_def("root"));
                            sb.push_without_newline(1, &param.set("root"));
                        }
                    }

                },
                MemberSource::Table(data) =>{
                    sb.push_without_newline(1, &data.get());
                },
                MemberSource::CList(l) =>{
                    sb.push_without_newline(1, &l.get());
                },
                MemberSource::MList(m) =>{
                    sb.push_without_newline(1, &m.get());
                },
                MemberSource::Cil(_) =>{},
                MemberSource::Mil(_) =>{},
            }
        }
        sb.push(0, "}");

        for mem in self.members(){
            match mem{
                MemberSource::Table(data) =>{
                    sb.push(0, &data.to_string())
                },
                MemberSource::CList(l) =>{
                    sb.push(0, &l.to_string())
                },
                MemberSource::MList(m) =>{
                    sb.push(0, &m.to_string())
                },
                MemberSource::Param(_) =>{},
                MemberSource::Cil(_) =>{},
                MemberSource::Mil(_)=>{},
            }
        }

        sb.to_string()
    }

    pub fn to_string_with_cfg_test(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push(0, &format!("#[cfg(test)] pub mod test{{"));
        sb.push(1, &self.to_string());
        sb.push(0, "}");
        sb.to_string()
    }
}