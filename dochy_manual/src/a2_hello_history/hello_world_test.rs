use dochy::error::DpResult;
use dochy::core::json_dir_to_root;
use crate::a1_hello_world::hello_world_accessor::RootIntf;

#[test]
fn hello_world_test() -> DpResult<()> {
    let root = json_dir_to_root("src/a1_hello_world/some_dir", true)?;

    let mut root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello World");
    root.set_message("Hello the next world".to_string());
    assert_eq!(root.message(), "Hello the next world");
    Ok(())
}