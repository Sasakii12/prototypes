#include <stdio.h>
#include <stdlib.h>

void birthday(int* age) {
    (*age)++;
}

int main() {
    // int age = 25;
    // int *p_age = &age;

    // printf("%p\n", p_age);
    // printf("%p\n", &age);

    // printf("%d\n", age);
    // birthday(p_age);
    // printf("%d\n", age);
    int number = 0;
    printf("Enter the number of grades: ");
    scanf("%d", &number);

    char *grades = malloc(number * sizeof(char));

    if (grades == NULL) {
        printf("Memory Allocation failed\n");
        return 1;
    }

    for (int i = 0; i < number; i++) {
        printf("Enter grade #%d: ", i + 1);
        scanf(" %c", &grades[i]);
    }

    for (int i = 0; i < number; i++) {
        printf("%c ", grades[i]);
    }

    free(grades);
    grades = NULL; // wahoo no dangling pointer
}