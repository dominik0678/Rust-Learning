# This layout has to be used, so it can function (or smth like that).
## The C lib files "have" (should, but doesn't need to [I think]) to be in the same directory as the main.rs. 
## If all files are in the correct directory, you/I can just simply do these steps:
- gcc -c -o src/simple_math.o src/simple_math.c (where the first one is the new file and the other the aimed file [where it should generate from])
- ar rcs lib/simple_math.a src/simple_math.o (again just like the first command)
- cargo run (should work [I hope so])

Also the build.rs is important so it knows to compile the C code.