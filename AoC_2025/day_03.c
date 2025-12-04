#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_LINES 200
#define MAX_LENGTH 200

int part_1(char data[MAX_LINES][MAX_LENGTH], int size, int line_size){
    int sum=0;

    for (int i = 0; i < size; i++){
        int largest_tens = 0;
        int pos_largest_ten = 0;
        int largest_ones = 0;
        // printf("%d\n", data[i][0] - '0');

        for (int n = 0; n < line_size; n++){
            if (largest_tens != 9 && largest_tens < data[i][n] - '0' && n != line_size - 1){
                largest_tens = data[i][n] - '0';
                pos_largest_ten = n;
                largest_ones = 0;
            }
            if (largest_ones != 9 && largest_ones < data[i][n] - '0' && n > pos_largest_ten){
                largest_ones = data[i][n] - '0';
            }
        }
        sum += largest_ones + largest_tens * 10;
    }
    return sum;
}

long long part_2(char data[MAX_LINES][MAX_LENGTH], int size, int line_size){
    // char sum[50];
    long long sum = 0;
    for (int i = 0; i < size; i++){
        long long multiplier = 100000000000LL;
        int pos_largest = -1;
        int off = line_size - 12;
        long long largest_total = 0LL;
        for (int x = 0; x < 12; x++){
            int largest = 0;
            int start = pos_largest + 1;
            int end = start + 1 + off;
            if (end > line_size)
                end = line_size;
            for (int n = start; n < end; n++){
                if (largest == 9){
                    continue;
                }
                if (largest < data[i][n] - '0' && n != line_size){
                    largest = data[i][n] - '0';
                    pos_largest = n;
                }
            }
            sum += (long long) largest * multiplier;
            largest_total += (long long) largest * multiplier;
            multiplier /= 10;
            if (off > 0){
                off -= (pos_largest - start);
            }
            
        }
    }
    return sum;
}

int main(){
    // TODO: get it to work on dynamical text sizes
    FILE *f = fopen("day_03.txt", "r");
    if (!f) {
        perror("Failed to open file");
        return 1;
    }

    char data[MAX_LINES][MAX_LENGTH];
    int size = 0;
    int line_size = 0;
    while (fgets(data[size], MAX_LENGTH, f)) {
        if (line_size == 0){
            line_size = strcspn(data[size], "\n");
        }
        data[size][line_size] = '\0';
        size++;
    }

    printf("part 1: %d\n", part_1(data, size, line_size));
    printf("part 2: %lld\n", part_2(data, size, line_size));
    
    // free(data);
    return 0;
}