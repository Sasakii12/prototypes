#include <stdio.h>
#include <math.h>


int main() {
    int x = 9;
    
    float sqrx = sqrt(x);
    int powx = pow(x, 2);

    float y = 1.4;
    int roundy = round(y);
    int ciely = ceil(y);

    // more math functions you get the idea

    printf("%d\n", ciely);
}