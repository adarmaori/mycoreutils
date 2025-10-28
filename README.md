# Mycoreutils - My attempt at understanding file and system operations

This is a toy project where I attempt to replicate some of the GNU coreutils with my own implementations in rust.
If you want to see an exmaple of this done well, I highly recommend checking out uutils/coreutils, where they actually care about mimicing the exact capacilities and behavior of the GNU implementations.

For me, it's just about learning things like system calls, buffered writing/reading, and space-efficient file handling.

Up till now, I've implemented the echo, cat, seq, yes, and head commands, with many of the flags and edge cases obviously missing.

The plan for the future is to continue discovering and implementing utils I find interesting, but the current todo's are more along the lines of writing tests and making clippy not want to kill me
