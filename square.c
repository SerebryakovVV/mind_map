#include "raylib.h"
#include <stdio.h>

typedef struct Point {
    float x;
    float y;
} Point;

void drawSquare(Point* vb[], int* ib, int vc) {
    for (int i = 0; i < vc; i++) {
        DrawLine(vb[ib[i]]->x, vb[ib[i]]->y, vb[ib[i+1]]->x, vb[ib[i+1]]->y, BLACK);
    }
}

void moveVertices(Point* vb[], int vc, int dir) {
    int dX = 0;
    int dY = 0;
    switch (dir) {
            case KEY_UP:
                dY = -10;
                break;
            case KEY_RIGHT:
                dX = 10;
                break;
            case KEY_DOWN:
                dY = 10;
                break;
            case KEY_LEFT:
                dX = -10;
                break;
    }
    for (int i = 0; i < vc; i++) {
        vb[i]->x += dX;
        vb[i]->y += dY;
    }
}


void moveVerticesMouse(Point* vb[], int vc, int dX, int dY) {
    for (int i = 0; i < vc; i++) {
        vb[i]->x += dX;
        vb[i]->y += dY;
    }
}

void zoom(Point* vb[], int vc, float zv, int mx, int my) {    
    for (int i = 0; i < vc; i++) {
        vb[i]->x = mx + (vb[i]->x - mx) * zv;
        vb[i]->y = my + (vb[i]->y - my) * zv;
    }
}

int main() {
    Point newPt1 = {100,100};
    Point newPt2 = {400,100};
    Point newPt3 = {400,400};
    Point newPt4 = {100,400};
    Point* vertixBuffer[] = {&newPt1, &newPt2, &newPt3, &newPt4};
    int indexBuffer[] = {0, 1, 2, 3, 0};
    int vertixCount = 4;
    float zoomValue;
    InitWindow(500, 500, "hello world title here");
    SetTargetFPS(60);
    int prevMX;
    int prevMY;
    int dX;
    int dY;
    int curX;
    int curY;
    while (!WindowShouldClose()) {
        if (IsKeyDown(KEY_UP)) moveVertices(vertixBuffer, vertixCount, KEY_UP);
        if (IsKeyDown(KEY_RIGHT)) moveVertices(vertixBuffer, vertixCount, KEY_RIGHT);
        if (IsKeyDown(KEY_DOWN)) moveVertices(vertixBuffer, vertixCount, KEY_DOWN);
        if (IsKeyDown(KEY_LEFT)) moveVertices(vertixBuffer, vertixCount, KEY_LEFT);
        zoomValue = GetMouseWheelMove();
        if (zoomValue != 0) {
            zoomValue = zoomValue > 0 ? 1.1 : 0.9;  
            zoom(vertixBuffer, vertixCount, zoomValue, GetMouseX(), GetMouseY());
        }
        if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
            prevMX = GetMouseX();
            prevMY = GetMouseY();
        }
        if (IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            curX = GetMouseX();
            curY = GetMouseY();
            dX = curX - prevMX;
            dY = curY - prevMY;
            prevMX = curX;
            prevMY = curY;
            moveVerticesMouse(vertixBuffer, vertixCount, dX, dY);
        }
        BeginDrawing();
        ClearBackground(RAYWHITE);
        drawSquare(vertixBuffer, indexBuffer, vertixCount);
        EndDrawing();
    }
    CloseWindow();
    return 0;
}