#include <stdio.h>
#include <string.h>


int main() {
    int age = 0;
    float gpa = 0.0f;
    char grade = '\0';
    char name[30] = "";

    printf("Please enter your age: ");
    scanf("%d", &age);

    printf("Please enter your gpa: ");
    scanf("%f", &gpa);

    printf("Please enter your grade: ");
    scanf(" %c", &grade);

    getchar();
    printf("Please enter your name: ");
    fgets(name, sizeof(name), stdin);
    name[strlen(name) - 1] = '\0';

    printf("%s\n", name);
    printf("%d\n", age);
    printf("%.2f\n", gpa);
    printf("%c\n", grade);
    printf("%s\n", name);

    printf("You are %d years old\n", age);
}