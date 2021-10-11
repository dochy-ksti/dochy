
Dochy Data can be used like a normal data file,
while Dochy History is intended to handle sequential (often small) changes,
like "undo", "auto-save", etc.

Basically, if you want to use Dochy History, you should also use
Dochy Data to save data permanently.
Otherwise, just retain every Dochy History file.
If you don't remove, it's permanent (but storage drives have their limits...).

Dochy History files have dependencies on previously saved files,
so removing single file is basically impossible, and retaining only single file is also impossible.

You must remove all dependants, and retain all dependencies.
This library has a removing policy, "retain last N files and dependencies".
Besides, you can specify files to retain and remove all the other files without their dependencies.

This is the typical directory composition.

─ proj_dir ┬─ history_dit
           ├─ save_dir
           └─ src_dir

Maybe you want to use multiple data for your project.

─ proj_dir ┬─ data_a_dir ┬─ history_dir_a
           │             ├─ save_dir_a
           │             └─ src_dir_a
           └─ data_b_dir ┬─ history_dir_b
                         ├─ save_dir_b
                         └─ src_dir_b

But you don't need to obey this.
There's no limitation about directory compositions in Dochy.
(but history_dir, save_dir, and src_dir need to be a dedicated directory.)