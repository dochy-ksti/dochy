## Hello History

"Dochy History" is a key feature of Dochy. 
When an application implements auto-save functionality,
changes from the last save tend to be small,
so Dochy's diff system fits in.

"Dochy History' employs a [complex algorithm](../sample_test/sample_code/history.md)
to construct diffs, but you just need to call "save_history_file"
and "load_history_file" to use it.

```json5
//root.json5
{
  data1 : "data1",
  data2 : "data2",
  data3 : "data3"
}
```
