## Reference and Polymorphism

Referencing something is difficult. 

In static programming languages, referents are retrieved by accessing memory address,
and in dynamic programming languages, by name-resolution.

Memory addresses are not serializable. 
Instead of them, MList automatically implements "ID" member which is auto-increment integers.
Since MList is implemented by a linked-hash-map, retrieving items by ID is extremely fast,
and it also has sequential order.

Referencing items with auto-increment ID is not very intuitive.
You can create very powerful systems on it with programming languages, 
but it's not very suited for handmade JSON5 files to reference items.

Dochy has another option, which is first-class reference based on name-resolution.
```
{
 list : [
  "MList",
  [{
   // "Ref" is the keyword to reference items
   // Every keyword in Dochy starts with captal letters,
   // and all the names starts with captal letters are reserved by the system
   Ref : {
    //Ref's initial value must be a ID, empty string, or null.
    //If it's an empty string, you must set ID, otherwise an error will occur.
    table_a : "", 
    // "table_a" is the table's name. The name of the variables must be the same
   },
   bar : -1,
  }],
  {
   Ref : {
    table : "item2", //reference item2 of the table below
    //The ID written here must exist, otherwise an error will occur.
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
  // The type of this collection is "Table", whose items can be referenced. 
  "Table",
  [{
   foo : 0
  }],
  {
   // ID is a keyword
   // All the items of a Table must have string IDs 
   ID : "item1",
   foo : 33,
  },
  {
   ID : "item2", // "item2" exists here
   //foo is not set
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
The "item1" is in the table.
```
{
 ID : "item1",
 foo : 33,
},
```
You can get data from references like this.
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
Table's items are immutable, (because I didn't implement "MTable", and I don't know whether it should exist.)
If you need to modify the value, you can use "nullable" values and wrappers.
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

```Rust
pub(crate) struct Ref2Wrapper{
    item : ListMItem
}

impl Ref2Wrapper {
    pub fn new(item: ListMItem) -> Ref2Wrapper { Ref2Wrapper { item } }

    pub fn foo(&self) -> i64{
        match self.item.foo(){
            NullOr::Null =>{
                // When it's null, the referenced value is returned
                self.item.ref_table().foo()
            },
            NullOr::Val(v) =>{
                // If it's updated, the updated value is returned
                v
            }
        }
    }
}
```