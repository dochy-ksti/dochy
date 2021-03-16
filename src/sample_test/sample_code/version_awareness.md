### Version Awareness

Dochy File System forces the diff files to be placed with 
 an archived source JSON5 file in the same directory.

When the source is modified, diff files will be invalid unless 
the source file at the time is preserved, and Dochy File System makes sure
the corresponding archive file is always preserved.

When the source is modified, we'll have the current and old source files.
Since we can get the old and current type data from them,
we can adjust the old data to be compatible with the current version.

In Dochy, If a variable is removed in a new version, 
the corresponding variable in the old version is also removed in the adjustment process, 
and it won't cause any problem.

What about adding a variable?

In Dochy, when we try to get a value from an object, 
and the object doesn't have the value, the default value is returned,
so when a variable is added, since the object of the old data doesn't have the value,
the default value is returned as always. It's basically safe.

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
 // every variable must have its type. ["Int", null] means the null's type is Int,
 // so the variable's type is "nullable Int"
 
 newValue! : 100, // ! means "can be undefined". It's also invalid in JSON5
 //newValue's default value is 100, which is 10 times bigger than the old.
}
 ```
This is the new version. oldValue's default value has been changed to null, 
so the oldValue should be null in the data from the new source if it's not updated,
and the value is old, it shouldn't be updated.
 
In data from the old source, if the oldValue is updated, 
the updated value remains unchanged, but the oldValue is not updated,
Dochy returns the new default value(null).

Since the old data doesn't have the variable "newValue", its value is 
changed to "undefined" in the adjustment process.

How to use the value "unchanged"?



