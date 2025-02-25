#include "raylib.h"
#include <stdio.h>

typedef struct Node {
    int x;
    int y;
    int height;
    int width;
} Node;


// get the strings from db
// get the width of a letter in a font
void drawMultilineString(char* wholeString, Node node) {
    // get the length of the string in pixels or something
    int l = MeasureText(wholeString, 10);

    // also pass the length of the string
    // insert the termination, then change back
    if (l > node.width) {
        char temp = wholeString[20];
        wholeString[20] = '\0';
        DrawText(wholeString, node.x, node.y, 10, BLACK);
        wholeString[20] = temp;
        DrawText((char*)(wholeString+20), node.x, node.y+30, 10, BLACK); // it is even working somehow
    } else {
        DrawText(wholeString, node.x, node.y, 10, BLACK);
    }
}

int main() {
 
    InitWindow(500, 500, "hello world title here");
    SetTargetFPS(60);
    Node task = {
        x: 50,
        y: 50,
        height: 300,
        width: 250
    };
    
    char taskStringShort[] = "short task";
    char taskStringLong[] = "this is a task and it is somewhat long";
    char taskStringVeryLong[] = "this is a task and it is somewhat long, actually it is very long";

    int l = MeasureText(taskStringVeryLong, 10);
    printf("%d", l);

    while (!WindowShouldClose()) {
      
        BeginDrawing();
        ClearBackground(RAYWHITE);

        DrawRectangle(task.x, task.y, task.width, task.height, BLUE);
        // DrawText(taskStringLong, 10, 10, 10, BLACK);
        // drawMultilineString(taskStringShort, task);
        drawMultilineString(taskStringVeryLong, task);
        

        EndDrawing();
    }
    CloseWindow();
    return 0;
}

// MeasureText, MeasureTextEx

