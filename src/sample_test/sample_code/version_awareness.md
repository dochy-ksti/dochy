### Version Awareness

Dochy File System forces the diff files to be placed with 
 an archived source JSON5 file in the same directory.

When the source is modified, diff files will be invalid unless 
the source file at the time is preserved, and Dochy File System makes sure
the corresponding archive file is always preserved.

When the source is modified, we'll have the current and old source files.
Since we can get the old and current type data from them,
we could adjust the old data to be compatible with the current version. 

