### Version Awareness

Dochy File System forces the diff files to be placed with 
 an archived source JSON5 file in the same directory.

When the source is modified, diff files will be invalid unless 
the source file at the time is preserved, and Dochy File System makes sure
the corresponding archive file is always preserved in the same directory.

When the source is modified, we'll have the current and old source files.
Since we can get the old and current type data from them,
we can adjust the old data to be compatible with the current version.

In Dochy, if a variable is removed in a new version, 
the corresponding variable in the old data is also removed in the adjustment process, 
and it shouldn't cause any problem.

What about adding a variable?

In Dochy, when we try to get a value from a variable, 
and a value has not been set to the variable, the default value is returned.
When a variable is added in the new version, since the object of the old data doesn't have the variable,
the default value is returned. It's basically safe, but sometimes, it's not.

Let's suppose we need a variable which is ten times bigger than an old variable.

How should we implement the conversion. We can write,

"If the data is old, multiply the variable by ten and update the variable."

But is that the right way? Accumulation of the conversion may cause problems in the future.

In Dochy, when a variable is undefined in an old data, it can set the special value "undefined" to the variable.

```json5
{
 oldValue : 10
}
```
 This is the old source file.
```json5
{
 // ? means "nullable". It's not valid in JSON5, but Dochy's parser accepts the syntax
 // like normal JSON, you can quote the member name "oldValue?" and it makes the name legal for JSON5(and JSON).   
 oldValue? : ["Int", null], 
 // every variable must have its type. and "null" type of a variable is prohibited (and meaningless)
 // ["Int", null] means the null's type is Int, so the variable's type is "nullable Int"
 
 newValue! : 100, // ! means "can be undefined". It's also invalid in JSON5
 //newValue's default value is 100, which is 10 times bigger than the old.
}
 ```
This is the new version. oldValue's default value has been changed to null, 
so the oldValue should be null in the data from the new source if it's not updated,
(and the variable is old, it shouldn't be updated.)
 
In data from the old source, if the oldValue is updated, 
the updated value remains unchanged, but the oldValue is not updated,
Dochy returns the new default value(null).

Since the old data doesn't have the variable "newValue", its value is 
changed to "undefined" in the adjustment process.

## How to use the value "unchanged"?

Dochy generates a source code to access the data. 
The generated code from the new source is this. 
(You don't need to read. It's just a generated code)
```Rust
use dochy_core::intf::*;
use dochy_core::structs::*;
unsafe impl Send for RootIntf{}
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

	pub fn old_value(&self) -> NullOr<i64>{
		let qv = root::get_int(self.ptr, "oldValue").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn old_value_def_val(&self) -> NullOr<i64>{
		let qv = root::get_int_def(self.ptr, "oldValue").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_old_value(&mut self, old_value : NullOr<i64>){
		root::set_int(self.ptr, "oldValue", old_value.into_qv());
	}
	pub fn new_value(&self) -> UndefOr<i64>{
		let qv = root::get_int(self.ptr, "newValue").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn new_value_def_val(&self) -> UndefOr<i64>{
		let qv = root::get_int_def(self.ptr, "newValue").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn set_new_value(&mut self, new_value : UndefOr<i64>){
		root::set_int(self.ptr, "newValue", new_value.into_qv());
	}
}
```
You can use the generated code directly, but a handmade wrapper is 
supposed to be made. This time, the wrapper is...
```Rust
use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use dochy_core::structs::{UndefOr, NullOr};
use std::cell::UnsafeCell;

struct VeraAccessorWrapper{
    //UnsafeCell can be used to mutate a variable over an immutable reference
    cell : UnsafeCell<RootIntf>
}

impl VeraAccessorWrapper{
    pub fn new_value(&self) -> i64{
        let root = unsafe{ &mut *self.cell.get() };

        match root.new_value(){
            UndefOr::Undefined =>{
                match root.old_value(){
                    NullOr::Null => root.new_value_def_val().into_value().unwrap(),
                    NullOr::Val(v) =>{
                        root.set_new_value(UndefOr::Val(v * 10));
                        root.new_value()
                    }
                }
            },
            UndefOr::Val(v) =>{
                v
            }
        }
    }
}
```


