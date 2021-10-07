Let's save data of Dochy.

There's two types of save files in Dochy, 
"Dochy Data" and "Dochy History".

Dochy Data saves the difference from "Dochy Src".

Simply put, Dochy History saves the difference from the previous save file.

Dochy Data files can be used by itself(and Dochy Src), 
while Dochy History files basically need multiple save files(and Dochy Src).

Dochy Data can be used like a normal data file,
while Dochy History handles sequential (often small) changes,
like "undo", "auto-save", etc.

Basically, if you want to use Dochy History, you should also use
Dochy Data to save data permanently. 
Otherwise, just retain every Dochy History file. 
If you don't remove, it's permanent (but storage drives have its limits...).
Dochy History files have dependencies on previously saved files, 
so removing single file is basically impossible, and retaining single file is also impossible.

You must remove all dependants, and retain all dependencies.
This library has only one removing policy, "retain last N files and dependencies".

