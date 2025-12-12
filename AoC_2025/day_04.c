#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int solution(char **list, size_t size, int part_2){
    int rolls = 0;
    int line_size = strlen(list[0]);
    for (int i=0; i<size; i++){
        for (int j=0; j<line_size; j++){
            if (strchr("@", list[i][j])){
                int i_min = i > 0 ? -1 : 0; 
                int i_max = i < size -1? 2 : 1;
                int j_min = j > 0 ? -1 : 0; 
                int j_max = j < line_size -1? 2 : 1;
                int adjacent = 0;
                for (int i_check = i_min; i_check < i_max; i_check++) {
                    for (int j_check = j_min; j_check < j_max; j_check++) {
                        if (adjacent >= 4)
                            break;
                        if (strchr("@", list[i+i_check][j+j_check]) && !(j_check == 0 && i_check == 0))
                            adjacent++;
                    }
                }
                if (adjacent < 4){
                    rolls++;
                    if (part_2)
                        list[i][j] = '.';
                } 
            } 
        }
    }
    if (!part_2)
        return rolls;
    else {
        if (rolls == 0)
            return rolls;
        else
            return rolls + solution(list, size, 1);
    }
}

int main(){
    FILE *fptr = fopen("day_04.txt", "r");

    if (!fptr){
        perror("Error opening file");
        return 1;
    }

    char **list = NULL;
    size_t size = 0;
    char buffer[256];

    while (fgets(buffer, sizeof(buffer), fptr)) {
        buffer[strcspn(buffer, "\n")] = '\0';
        list = realloc(list, (size + 1) * sizeof(char *));
        list[size] = malloc(strlen(buffer) + 1);
        strcpy(list[size], buffer);
        size++;
    }

    printf("part 1: %d\n", solution(list, size, 0));
    printf("part 2: %d\n", solution(list, size, 1));

    return 0;
}