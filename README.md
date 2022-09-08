# pixo2ecas

This is a small project which aims at converting the exported csv file from [pixometer](https://play.google.com/store/apps/details?id=com.pixolus.pixometer) to an importable file format for the [ecas-app](https://play.google.com/store/apps/details?id=at.topfen.ecas).
pixometer exports every meter in one file and distinguishes them by one name column. Ecas on the other hand lets you only import one file per meter at a time and not import many meters at once. This is taken care of by splitting the input file into one file per meter found in the pixometer export.

This project also is an introduction of myself to rust and the polars library.

## How to build and run

Install rust and run
```
cargo build
```
then
```
cargo run <path/to/file.csv>
```
to run it and let it convert the input csv file to one output csv file per meter found inside the csv file.
If the argument is omitted, the example file from the test/res folder is used.
