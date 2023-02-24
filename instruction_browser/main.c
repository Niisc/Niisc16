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



        {{0,0,0,0,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "ADD", {2,5}},
        {{0,0,0,0,0,0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2}, "AX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2}, "AX", {10,13}},
        {{0,0,0,0,0,0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2}, "AS", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 0, 0, 1, 2, 2}, "AS", {10,13}},
        {{0,0,0,0,0,0, 0, 0, 1, 0, 2, 2, 2, 2, 2, 2}, "BX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 0, 1, 0, 2, 2}, "BX", {10,13}},
        {{0,0,0,0,0,0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2}, "BS", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 0, 1, 1, 2, 2}, "BS", {10,13}},
        {{0,0,0,0,0,0, 0, 1, 0, 0, 2, 2, 2, 2, 2, 2}, "CX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 1, 0, 0, 2, 2}, "CX", {10,13}},
        {{0,0,0,0,0,0, 0, 1, 0, 1, 2, 2, 2, 2, 2, 2}, "CS", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 1, 0, 1, 2, 2}, "CS", {10,13}},
        {{0,0,0,0,0,0, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2}, "DX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 1, 1, 0, 2, 2}, "DX", {10,13}},
        {{0,0,0,0,0,0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 2}, "DS", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 0, 1, 1, 1, 2, 2}, "DS", {10,13}},
        {{0,0,0,0,0,0, 1, 0, 0, 0, 2, 2, 2, 2, 2, 2}, "EX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 0, 0, 0, 2, 2}, "EX", {10,13}},
        {{0,0,0,0,0,0, 1, 0, 0, 1, 2, 2, 2, 2, 2, 2}, "ES", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 0, 0, 1, 2, 2}, "ES", {10,13}},
        {{0,0,0,0,0,0, 1, 0, 1, 0, 2, 2, 2, 2, 2, 2}, "FX", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 0, 1, 0, 2, 2}, "FX", {10,13}},
        {{0,0,0,0,0,0, 1, 0, 1, 1, 2, 2, 2, 2, 2, 2}, "FS", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 0, 1, 1, 2, 2}, "FS", {10,13}},

        {{0,0,0,0,0,0, 1, 1, 0, 0, 2, 2, 2, 2, 2, 2}, "Ret Addr", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 1, 0, 0, 2, 2}, "Ret Addr", {10,13}},
        {{0,0,0,0,0,0, 1, 1, 0, 1, 2, 2, 2, 2, 2, 2}, "SP", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 1, 0, 1, 2, 2}, "SP", {10,13}},

        {{0,0,0,0,0,0, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2}, "IP", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 1, 1, 0, 2, 2}, "IP", {10,13}},
        {{0,0,0,0,0,0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2}, "Output", {6,9}},
        {{0,0,0,0,0,0, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2}, "Input", {10,13}},

        {{0,0,0,0,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "SUB", {2,5}},
        {{0,0,0,0,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "AND", {2,5}},
        {{0,0,0,0,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "OR", {2,5}},
        {{0,0,0,1,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "NOT", {2,5}},
        {{0,0,0,1,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "XOR", {2,5}},
        {{0,0,0,1,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "MUL", {2,5}},
        {{0,0,0,1,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "DIV", {2,5}},
        {{0,0,1,0,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "MOD", {2,5}},
        {{0,0,1,0,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "SHL", {2,5}},
        {{0,0,1,0,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "SHR", {2,5}},
        {{0,0,1,0,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LESW", {2,5}},
        {{0,0,1,1,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LOEW", {2,5}},
        {{0,0,1,1,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LNOT", {2,5}},
        {{0,0,1,1,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LESB", {2,5}},
        {{0,0,1,1,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LOEB", {2,5}},



        {{1,0,0,0,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JMP", {2,5}},
        {{1,0,0,0,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JZ", {2,5}},
        {{1,0,0,0,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JNZ", {2,5}},
        {{1,0,0,0,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JPW", {2,5}},
        {{1,0,0,1,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JNW", {2,5}},
        {{1,0,0,1,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JPB", {2,5}},
        {{1,0,0,1,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "JNB", {2,5}},



        {{1,1,0,0,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "PUSHW", {2,5}},
        {{1,1,0,0,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "POPW", {2,5}},
        {{1,1,0,0,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "CALL", {2,5}},
        {{1,1,0,0,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "RET", {2,5}},
        {{1,1,0,1,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LDR", {2,5}},
        {{1,1,0,1,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "STR", {2,5}},
        {{1,1,0,1,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "MOV", {2,5}},
        {{1,1,0,1,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "NOP", {2,5}},
        {{1,1,1,0,0,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "PUSHB", {2,5}},
        {{1,1,1,0,0,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "POPB", {2,5}},
        {{1,1,1,0,1,0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "LDRB", {2,5}},
        {{1,1,1,0,1,1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}, "STRB", {2,5}},
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

    uint16_t num, width = 0;
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

        DrawText("Instructions", screenWidth/2-110,40,36,WHITE);
        num = 0;
        for (int i = 0; i < 16; ++i) {
            num += bits[15-i].state * pow(2,i);
        }

        width = MeasureText(TextFormat("%d",num),36);
        DrawText(TextFormat("%d",num), (screenWidth/2-width/2),220, 36, WHITE);

        for (int i = 0; i < 39; ++i) {
            if (is_rule_applied(i)){
                Vector2 *line_pos = bit_to_vec2(instructions[i].highlight_bit);
                DrawLineEx(line_pos[0],line_pos[1], 2.5f, ORANGE);
                width = MeasureText(TextFormat("%s",instructions[i].name),30);
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
    if (!vectors) {
        printf("Mika stop eating ram ples");
        exit(1);
    }
    vectors[0] = (Vector2){c_bits[0] * (RADIUS*2+16) + 60,900/6 - 30};
    vectors[1] = (Vector2){c_bits[1] * (RADIUS*2+16) + 60,900/6 - 30};
    return vectors;
}
