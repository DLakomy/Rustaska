# Rustaska

Remake of https://github.com/DLakomy/Alaska. My first project in Rust.

TODO:
- [X] reading the source record by record
- [X] parser (quality of its error messages are not important right now)
- [X] persistence
- [X] e2e test
- [X] simple performance test DONE. I've used the `release` build, took a `genExampleFile 20000000 bigExample.lst` result from Alaska, and it took (measured with `time`)... 85 seconds :/ Could be faster, but dunno how ¯\\_(ツ)_/¯
