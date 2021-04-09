## Reference

Referencing something is difficult. 

In static programming languages, referents are retrieved by accessing memory address,
and in dynamic programming languages, by name-resolution.

Memory addresses are not serializable. 
Instead of them, MList automatically implements "ID" member, which is auto-increment integers.
Since MList is implemented by a linked-hash-map, retrieving items by ID is extremely fast,
and it also remains sequential order.

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
    tableA : "", 
    // "tableA" is the table's name.
   },
   bar : -1,
  }],
  {
   Ref : {
    tableA : "item2", //reference item2 of the tableA
    //The ID written here must exist, otherwise an error will occur.
   },
   //bar is not set
  },
  {
   Ref : {
    tableA : "item1"
   },
   bar : 30,
  }
 ],
 //This is the "tableA" 
 tableA : [
  // The type of this collection is "Table", whose items can be referenced. 
  "Table",
  [{
   foo : 0
  }],
  {
   // ID is a keyword
   // All the items of a Table must have a string ID 
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
  tableA : "item1"
 },
}
```
This part is the "Ref". The item references "item1".
The "item1" is in the table.
```
{
 ID : "item1",
 foo : 33,
},
```
You can get data from references like this.
```Rust
#[test]
fn ref1_test() -> DpResult<()> {
    let old = json_dir_to_root("src/sample_test/sample_code_json/ref1", true)?;

    let mut r = RootIntf::new(old);
    let mut list = r.list();
    //mlist is linked-hash-map, which is hashtable whose items are doubly-linked-list-node.
    //so first() and last() can be done instantly, but getting middle items are slow,
    //unless you use find_by_id instead.
    //Linked-hash-map is also hashmap, so you can find items by key(ID) super fast.
    let item = list.last()?;
    assert_eq!(item.ref_table_a().foo(), 33);
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
        tableA : "",
      },
      "foo?" : ["Int",null],
    }],
    {
      Ref : {
        tableA : "item1",
      },
      // The value is updated here
      foo : 22,
    },
  ],
  tableA : [
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
                self.item.ref_table_a().foo()
            },
            NullOr::Val(v) =>{
                // If it's updated, the updated value is returned
                v
            }
        }
    }
}
```
If it's null, the referent's value is returned.
If you need to modify the value, set the item's variable other than null.

### Multiple Reference

Let's make some RPG. In the game, player-characters have two properties, "race" and "class".

We can write it with "multiple reference".

```json5
{
  pcList : [
    "MList",
    [{
      Ref : {
        //two references, "race" and "class"
        race : "",
        class : "",
      },
      name : "",
    }],
    {
      Ref : {
        //Elves are smart, so Mage is good for them
        race : "elf",
        class : "mage",
      },
      name : "Elvis",
    },
    {
      Ref : {
        //Dwarves are powerful, so Fighter is good for them
        race : "dwarf",
        class : "fighter",
      },
      name : "George",
    },
  ],
  race : [
    "Table",
    [{
      strength : 0,
      intelligence : 0,
    }],
    {
      ID : "elf",
      strength : 35,
      intelligence : 50,
    },
    {
      ID : "dwarf",
      strength : 55,
      intelligence : 30,
    }
  ],
  class : [
    "Table",
    [{
      attack : 0,
      magic : 0,
    }],
    {
      ID : "fighter",
      attack : 50,
      magic : 0,
    },
    {
      ID : "mage",
      attack : 5,
      magic : 30,
    }
  ]
}
```
You may think the modeling in this example is natural.

### Enum

Let's make the characters have items.
In this example, there are two kinds of items, swords and herbs.
```json5
//root.json5
{
  pcList : [
    "MList",
    //MList's default object is defined here
    [{
      name : "",
      items : [
        // "Mil" stands for "Mut Inner List"
        // "MilDef" means "Mut-Inner-List's Default Object"
        "MilDef",
        [{
          //"Enum" is a keyword
          // Enum is basically the programming language Rust's "enum"
          Enum : {
            //Enum's variables must be nullable, and the default values must be null
            "sword?" : null,
            "herb?" : null,
          }
        }]
        //No items can be written in a default object's inner list.
      ]
    }],
    {
      name : "Elvis",
      //Elvis has a bronze sword and a middle herb
      items : [
        //Mil stands for "Mut Inner List"
        "Mil",
        //We need to write inner list's items without a default object.
        //(The default object is already written above)
        {
          Enum : {
            //You must set only one variable to define Enum,
            sword : "bronze",
          }
        },
        {
          Enum : {
            herb : "middle",
          }
        }
      ]
    },
    {
      name : "George",
      //George has a low herb and an iron sword
      items : [
        "Mil",
        {
          Enum : {
            herb : "low",
          }
        },
        {
          Enum : {
            sword : "iron",
          }
        },
      ]
    },
  ],
}
```
We also need "sword" and "herb" tables.
We can write top level items in separate files.
We need to place these files in the directory "root.json5" exists.
```json5
//herb.json5
[
  "Table",
  [{
    restore : 0,
  }],
  {
    ID : "low",
    restore : 30,
  },
  {
    ID : "middle",
    restore : 80,
  }
],
```
```json5
//sword.json5
[
  "Table",
  [{
    attack : 0,
    price : 0,
  }],
  {
    ID : "bronze",
    attack : 10,
    price : 100,
  },
  {
    ID : "iron",
    attack : 40,
    price :500,
  }
]
```
Only top level items can be written in separate files.
