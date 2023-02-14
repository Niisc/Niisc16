#include "include/raylib.h"
#include <stdio.h>
#include <iostream>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <math.h>
#include <map>
#include <array>



struct Mode_pair {
    std::array<bool, 2> bits;
    std::string mode;
};

struct Bit {
    bool state;
    Vector2 pos;
};

int main(void)
{
    const int screenWidth = 1440;
    const int screenHeight = 900/2;
    
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(screenWidth, screenHeight, "market pathfinder");

    Mode_pair mode_pairs[] = {
            {{false, false}, "ALU"},
            {{false, true}, "IMM"},
            {{true, false}, "COND"},
            {{true, true}, "MISC"},
    };


    SetTargetFPS(60);

    while (!WindowShouldClose())
    {

        BeginDrawing();

            ClearBackground((Color){48,48,54});
            DrawText(TextFormat("Hello i am testing this, x: %.4f, y:%.4f  pessed: %d", GetMouseX(), GetMouseY(), IsMouseButtonPressed(MOUSE_BUTTON_LEFT)), 10, 10, 20, (Color){ 255, 255, 255, 255 });
            DrawCircle(10,10,20.0f,(Color){ 255, 255, 255, 255 });

        EndDrawing();
    }

    CloseWindow();

    return 0;
}

