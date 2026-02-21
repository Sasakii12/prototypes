#include <stdio.h>
#include <string.h>

int main() {
    char item[20] = "";
    int pizza_amt = 0;
    float price = 0.0f;

    printf("What item would you like to buy?: ");
    fgets(item, sizeof(item), stdin);
    item[strlen(item) - 1] = '\0';

    printf("What is the price for each?: ");
    scanf("%f", &price);

    printf("How many would you like?: ");
    scanf(" %d", &pizza_amt);

    printf("You have bought %d %s\n", pizza_amt, item);
    printf("The total is: $%.2f\n", price * (float)pizza_amt);
}