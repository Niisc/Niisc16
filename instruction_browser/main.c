#include "include/raylib.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <math.h>


#define AMOUNT_OF_BITS 16 //16 bit architecture
#define RADIUS 20.0f

typedef enum State {
    off = 0,
    on,
    idk
} State;


typedef struct Rule {
    State states[AMOUNT_OF_BITS];
    char *name;
    uint8_t highlight_bit[2]; //draw a line from bit a to bit b
} Rule;

typedef struct Bit {
    uint8_t state;
    Vector2 pos;
} Bit;


Bit bits[AMOUNT_OF_BITS] = { 0 };

Rule instructions[] = {
        {{0,0,2,2,2,2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "ALU", {0,1}},
        {{0,1,2,2,2,2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "IMM", {0,1}},
        {{1,0,2,2,2,2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "COND", {0,1}},
        {{1,1,2,2,2,2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "MISC", {0,1}},

}; //maybe use heap instead to make adding instructions faster

uint8_t is_rule_applied(int pos);
Vector2 *bit_to_vec2(uint8_t* c_bits);


int main(void)
{
    for (int i = 0; i < 16; ++i) {

        bits[i].pos.y = 900/6;
        bits[i].pos.x = i * (RADIUS*2+16) + 60;
    }

    const int screenWidth = 1440*2/3;
    const int screenHeight = 900/3;

    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(screenWidth, screenHeight, "instruction browser");

    SetTargetFPS(60);

    while (!WindowShouldClose())
    {

        if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
            for (int i = 0; i < AMOUNT_OF_BITS; ++i) {
                if (CheckCollisionPointCircle(GetMousePosition(), bits[i].pos, RADIUS)) {
                    bits[i].state = !bits[i].state;
                    break;
                }
            }
        }

        BeginDrawing();


            ClearBackground((Color){48,48,54});


            for (int i = 0; i < 4; ++i) {
                if (is_rule_applied(i)){
                    Vector2 *line_pos = bit_to_vec2(instructions[i].highlight_bit);
                    DrawLineEx(line_pos[0],line_pos[1], 2.5f, ORANGE);
                    int width = MeasureText(TextFormat("%s",instructions[i].name),30);
                    DrawText(TextFormat("%s",instructions[i].name),((line_pos[1].x-line_pos[0].x)/2) + line_pos[0].x - width/2,900/6 - 60, 30, WHITE);
                }
            }

            for (int i = 0; i < AMOUNT_OF_BITS; ++i) {
                DrawCircleV(bits[i].pos, RADIUS, (bits[i].state ? GREEN :RED ));
            }

        EndDrawing();
    }

    CloseWindow();

    return 0;
}

uint8_t is_rule_applied(int pos) {
    for (int i = 0; i < AMOUNT_OF_BITS; ++i) {
        if (instructions[pos].states[i] == 2) { continue;}

        if (instructions[pos].states[i] != bits[i].state) {
            return 0; //false
        }

    }
    return 1; //true
}

Vector2 *bit_to_vec2(uint8_t* c_bits){
    Vector2 *vectors = malloc(sizeof(Vector2)*2);
    vectors[0] = (Vector2){c_bits[0] * (RADIUS*2+16) + 60,900/6 - 30};
    vectors[1] = (Vector2){c_bits[1] * (RADIUS*2+16) + 60,900/6 - 30};
    return vectors;
}