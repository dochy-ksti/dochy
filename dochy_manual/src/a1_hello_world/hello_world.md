## Hello World

Dochy is a language, so we need "Hello World".
```json5
//root.json5
{
  message : "Hello World"
}
```
Dochy's source files are written in JSON5 slightly customized for Dochy.
The file name must be "root.json5", 
and the dedicated directory to place this file is also needed.
```
some_dir-root.json5
```
Let's convert the source file into a Dochy's object.
```Rust
use dochy::error::DpResult; //DpResult is the Dochy's error type.
use dochy::core::structs::RootObject;
use dochy::core::json_dir_to_root;

#[test]
fn hello_world() -> DpResult<()> {
    let _root_obj : RootObject = json_dir_to_root("src/a1_hello_world/some_dir", true)?;
    Ok(())
}
```
The function "json_dir_to_root" converts the source file to "RootObject". 
Designating directory's path is needed.

"RootObject" has low-level interfaces, 
but generally you need to generate the source code to access the converted object.
```Rust

```