## Reference and Polymorphism

Referencing something is difficult. 

In static programming languages, referents are retrieved by accessing memory address,
and in dynamic programming languages, by name-resolution.

Memory addresses are not serializable. 
Instead of them, MList automatically implemlements "ID" member which is auto-increment integers.
Since MList is implemented by a linked-hash-map, retrieving items by ID is extremely fast,
and it also remains sequential order.

Referencing items with auto-increment ID is not very intuitive.
You can create very powerful systems on it with programming languages, 
but it's not very suitable for handmade JSON5 files to reference items.

Dochy has another option, which is first-class reference based on name-resolution.
```
{
 list : [
  "MList",
  [{
   Ref : {
    //Ref's initial value must be a ID, empty string, or null
    //If it's empty string, you must set ID, otherwise an error will occur.
    table : "", 
   },
   bar : -1,
  }],
  {
   Ref : {
    table : "item2", //reference item2 of the table below
    //IDs must be exists, otherwise an error will occur.
   },
   //bar is not set
  },
  {
   Ref : {
    table : "item1"
   },
   bar : 30,
  }
 ],
 table : [
  "Table",
  [{
   foo : 0
  }],
  {
   ID : "item1",
   foo : 33,
  },
  {
   ID : "item2",
   //not set
  }
 ]
}
```
Tables are basically CList with string ID, 
and Dochy's objects can have "Ref", which can reference table's items by the ID.
```
{
 Ref : {
  table : "item1"
 },
}
```
This part is the "Ref". The item references the "item1".
```
{
 ID : "item1",
 foo : 33,
},
```
You can get the reference's data like this.
```Rust
fn ref1_test() -> DpResult<()> {
    let old = json_dir_to_rust("src/sample_test/sample_code_json/ref1", true)?;

    let mut r = RootIntf::new(old);
    let mut list = r.list();
    //MList is linked-hash-map, which is a hashtable whose items are doubly-linked-list-node,
    //so first() and last() can be done instantly, but getting middle items are slow because it is a linked-list,
    //but it's also a hashtable, so you can find items by key(ID) super fast.
    //If you know ID of the middle item, you can get the item instantly.
    let item = list.last()?;
    assert_eq!(item.ref_table().foo(), 33); //the referenced item's "foo" is 33
    Ok(())
}
```
Table's items are immutable. If you need to modify the value, 
you can use "nullable value" and wrappers.
```
{
 list : [
  "MList",
  [{
   Ref : {
    table : "", 
   },
   foo? : null,
  }],
  {
   Ref : {
    table : "item1", 
   },
   foo : 22,
  },
 ],
 table : [
  "Table",
  [{
   foo : 0
  }],
  {
   ID : "item1",
   foo : 33,
  },
 ]
}
```