*** I'm Not An English Speaker. Please Correct My English ***

Dochy is a static JSON-like data format.

It can efficiently store "diff" of the data. 

It's designed to implement auto-save, undo, and 
applications which must retain every change of the data. 

*Demonstration

[Demo](https://github.com/dochy-ksti/dochy_bench)

Test Data
```JSON5
{
  "data0": "oiufsjdsj...", //1 MB of random string
  "data1": "abuisoehg...", //1 MB of random string
  //...
  "data9": "bhsiofdis...", //1 MB of random string
}
```
The JSON has ten random 1 MB strings, so the entire JSON file is about 10 MB.

We modify one string and save at a time. We repeat it 100 times.

It means 10% of the data is modified each time.

Data is partially modified and saved most of the time.
So it's not very unrealistic settings, I think.

This demo creates a hundred files having 10 MB of data, so
the total file size is about 1 GB.

We prepared the equivalent Dochy data.