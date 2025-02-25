#include "raylib.h"
#include <stdio.h>

typedef struct Node {
    int x;
    int y;
    int height;
    int width;
} Node;

void drawMultilineString(char* wholeString, Node node) {
    // get the length of the string in pixels or something
    DrawText(wholeString, node.x, node.y, 10, BLACK);
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

    while (!WindowShouldClose()) {
      
        BeginDrawing();
        ClearBackground(RAYWHITE);

        DrawRectangle(task.x, task.y, task.width, task.height, BLUE);
        DrawText(taskStringLong, 10, 10, 10, BLACK);
        drawMultilineString(taskStringShort, task);
        

        EndDrawing();
    }
    CloseWindow();
    return 0;
}

// MeasureText, MeasureTextEx

