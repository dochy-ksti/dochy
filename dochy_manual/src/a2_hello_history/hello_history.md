## Hello History

"Dochy-History" is a key feature of Dochy. 
When an app implements auto-save functionality,
changes from the last save tend to be small,
so Dochy's diff system fits in.

"Dochy-History' employs a [complex algorithm](../sample_test/sample_code/history.md)
to construct diff efficiently, but you need to just call "save_history_file"
and "load_history_file" to use it.

```json5
//root.json5
{
  very_big_data : "This is very big data(kind of), but it's immutable so you don't need to save",
  small_data : "This data is small, but changes frequently"
}
```
