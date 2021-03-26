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
```
{
 // ? means "nullable". It's not valid syntax in JSON5, but Dochy's parser accepts it
 // like normal JSON, you can quote the member name "oldValue?" and makes the name legal    
 oldValue? : ["Int", null], 
 // every variable must have its static type. and the static type "null" is prohibited (and meaningless)
 // ["Int", null] means the null's type is Int, and the variable's static type is "nullable Int"
 
 newValue! : 100, // ! means "can be undefined". It's also invalid in JSON5 syntax but my parser accepts it.
 //newValue's default value is 100, which is 10 times bigger than the old.
}
 ```
This is the new version. oldValue's default value has been changed to null, 
so Dochy returns null when we get the oldValue if it's not updated.
(and it shouldn't be updated because the oldValue is old).
 
In data from the old source, if the oldValue was updated, 
the updated value remains unchanged. When the oldValue is not updated,
Dochy returns the new default value(null) even though it's origin is the old source.

Since the old data doesn't have the variable "newValue", the content of the variable is 
changed to "undefined" in the adjustment process.

## How to use the value "undefined"?

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
You can use the generated code directly, but we assume a handmade wrapper will be created.
```Rust
use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use dochy_core::structs::{UndefOr, NullOr};

pub(crate) struct VeraAccessorWrapper{
 root : RootIntf
}

impl VeraAccessorWrapper {
 pub fn new(root: RootIntf) -> VeraAccessorWrapper { VeraAccessorWrapper { root } }

 pub fn new_value(&self) -> i64 {
  Self::new_value_impl(&self.root)
 }

 fn new_value_impl(root: &RootIntf) -> i64 {
  match root.new_value() {
   //If data is old, the variable "new_value" will be "undefined" because it was not defined at the time.
   UndefOr::Undefined => {
    match root.old_value(){
     NullOr::Null => root.new_value_def_val().into_value().unwrap(),
     NullOr::Val(v) => v * 10, //new_value is ten times bigger than the old value
    }
   },
   UndefOr::Val(v) => {
    v
   }
  }
 }
}
```
The wrapper doesn't allow to access the variable "oldValue", and when new_value is undefined (because the data is old),
the wrapper returns the value ten times bigger than the old_value.

If the newValue is undefined, and the oldValue is unmodified, it returns the default value.

Let's suppose we need to define a value again. We name it new_value2. The value is three times bigger than new_value,
and the default value is 280.
```
{
    oldValue? : ["Int", null],
    //!? means the value can be null or undefined
    newValue!? : ["Int", null],
    newValue2! : 280,
}
```
The wrapper of it is below.
```Rust
use crate::sample_test::sample_code::version_awareness_accessor2::RootIntf;
use dochy_core::structs::{UndefOr, NullOr, Qv};

pub(crate) struct VeraAccessorWrapper2 {
    root: RootIntf,
}

impl VeraAccessorWrapper2 {
    pub fn new(root: RootIntf) -> VeraAccessorWrapper2 { VeraAccessorWrapper2 { root } }

    pub fn new_value2(&self) -> i64 {
        Self::new_value2_impl(&self.root)
    }

    fn new_value_impl(root: &RootIntf) -> NullOr<i64> {
        match root.new_value() {
            // We call a value "Qv" which can be "null" or "undefined". Maybe Qv stands for "Questionable value"
            Qv::Undefined => {
                root.old_value().map(|&old| old * 10)
            },
            Qv::Null => NullOr::Null,
            Qv::Val(v) => { NullOr::Val(v) }
        }
    }

    fn new_value2_impl(root : &RootIntf) -> i64{
        match root.new_value2(){
            UndefOr::Undefined =>{
                match Self::new_value_impl(root){
                    //This value hasn't been modified yet. Returns the current default value.
                    NullOr::Null => root.new_value2_def_val().into_value().unwrap(),
                    //The value has been modified in the older version(s). Convert the value to the current version
                    NullOr::Val(v) =>{
                        v * 3
                    }
                }
            },
            UndefOr::Val(v) =>{
                v
            },
        }
    }
}
```
When new_value2 is undefined, new_value * 3 is returned if it's defined, and if it's undefined, old_value * 30 is returned,
but if old_value and new_value haven't been modified, the default value is returned.

You can see this is a sustainable conversion from the source code.

## Const List and Mutable List