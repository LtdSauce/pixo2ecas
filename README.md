# pixo2ecas

This is a small project which aims at converting the exported csv file from [pixometer]() to an importable file format for the [ecas-app]().
pixometer exports every meter in one file and distinguishes them by one name column. Ecas on the other hand lets you only import one file per meter at a time and not import many meters at once. This is taken care of by splitting the input file into one file per meter found in the pixometer export.

This project also is an introduction of myself to rust and the polars library.

## How to build and run

Install rust and run
```
cargo build
```
then
```
cargo run
```
to run it.