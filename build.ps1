gcc main.c -o main.exe -I include/ -L lib/ -lraylib -lopengl32 -lgdi32 -lwinmm
./main.exe

# $arg = $args[0]
# $sourceFile = "$arg.c"
# $outputFile = "$arg.exe"
# & gcc $sourceFile -o $outputFile -I include/ -L lib/ -lraylib -lopengl32 -lgdi32 -lwinmm
# & .\$outputFile

