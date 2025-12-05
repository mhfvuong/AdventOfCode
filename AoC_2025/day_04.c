#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int part_1(char **list, size_t size){
    int rolls = 0;
    int line_size = strlen(list[0]);
    for (int i=0; i<size; i++){
        for (int j=0; j<line_size; j++){
            // printf("%d\n", strchr("@", list[i][j]));
            if (strchr("@", list[i][j])){
                // printf("%c", list[i][j]);
                int i_min = i > 0 ? -1 : 0; 
                int i_max = i < size -1? 2 : 1;
                int j_min = j > 0 ? -1 : 0; 
                int j_max = j < line_size -1? 2 : 1;
                int adjacent = 0;
                // printf("found '@' at i=%d j=%d\n", i, j);
                for (int i_check = i_min; i_check < i_max; i_check++) {
                    for (int j_check = j_min; j_check < j_max; j_check++) {
                        if (adjacent >= 4)
                            break;
                        // if (!(j_check == 0 && i_check == 0)) {
                        //     printf("checking: i=%d j=%d\n", i+i_check,j+j_check);
                        // }
                        if (strchr("@", list[i+i_check][j+j_check]) && !(j_check == 0 && i_check == 0))
                            adjacent++;
                    }
                }
                if (adjacent < 4){
                    rolls++;
                    //printf("x");
                } //else
                    //printf("@");
            } //else 
                //printf(".");
        }
        printf("\n");
    }
    return rolls;
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

    printf("part 1: %d\n", part_1(list, size));

    return 0;
}