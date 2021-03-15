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
so a variable is added, since the object of the old data doesn't have the value,
the default value is returned as always. It's safe, most of the time.

Let's suppose we need a variable which is ten times bigger than an old variable.

How should we implement the conversion. We can write,

"If the data is old, multiply the variable by ten."

But is that the right way? Accumulation of the conversion may cause problems in the future.

In Dochy, when a variable is undefined in an old data, it can set the value "undefined" to the variable.


