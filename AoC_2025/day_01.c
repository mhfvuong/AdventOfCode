#include <stdio.h>
#include <stdlib.h>

int part1(int *data, int size){
    int zeros = 0;
    int position = 50;
    for (int i = 0; i < size; i++){
        int step = data[i];
        if (abs(data[i]) >= 100){
            step = abs(data[i]) % 100 * (data[i] < 0 ? -1 : 1);
        }
        position += step;
        if (position < 0) position += 100;
        if (position >= 100) position -= 100;
        if (position == 0) zeros++;
    }
    return zeros;
}

int part2(int *data, int size){
    int zeros = 0;
    int position = 50;
    for (int i = 0; i < size; i++){
        int step = data[i];
        int previous_position = position;
        if (abs(data[i]) >= 100){
            zeros += abs(data[i]) / 100;
            step = abs(data[i]) % 100 * (data[i] < 0 ? -1 : 1);
        }
        position += step;
        if (position < 0) {
            position += 100;
            if (position != 0 && previous_position != 0)
                zeros++;
        }
        if (position >= 100) {
            position -= 100;
            if (position != 0 && previous_position != 0)
                zeros++;
        }
        if (position == 0) zeros++;
    }
    return zeros;
}

int main(){
    FILE *f = fopen("day_01.txt", "r");
    if (!f) {
        perror("Failed to open file");
        return 1;
    }

    int capacity = 256;
    int size = 0;
    int *data = malloc(capacity * sizeof(int));
    if (!data) {
        perror("Memory allocation failed");
        fclose(f);
        return 1;
    }

    char line[64];

    while (fgets(line, sizeof(line), f)) {
        if (size >= capacity) { // Resize array if needed
            capacity *= 2;
            data = realloc(data, capacity * sizeof(int));
            if (!data) {
                perror("Memory reallocation failed");
                fclose(f);
                return 1;
            }
        }

        char direction = line[0];
        int value = atoi(&line[1]);
        data[size++] = (direction == 'L') ? -value : value;
    }

    fclose(f);
    printf("Part 1: %d\n", part1(data, size));
    printf("Part 2: %d\n", part2(data, size));
    free(data);
    return 0;
}