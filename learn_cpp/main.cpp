#include <cmath>
#include <iomanip>
#include <iostream>
using namespace std;

int factorial(int x);


double pi(int x) {
    double digits = x;
    double sum = 0.;
    for (int i = 0; i < digits; i++) {
        double i_double = static_cast<double>(i);
        double num = static_cast<double>(pow(-1, i)) * factorial(6 * i) * (545140134 * i + 13591409);
        double denom = static_cast<double>(factorial(3 * i) * pow(factorial(i), 3)) * static_cast<double>(powf(640320, 3. * i_double + (3. / 2.))) ;
        sum += num / denom;
    }
    return 1 / (12.0 * sum);
}

int factorial(int x) {
    if (x == 0) {
        return 1;
    }
    return x * factorial(x-1);
}

int main() {
    int x = 0;
    cout << "Please enter a number\n";
    cin >> x;
    cout << "PI: " << setprecision(x) << pi(x);
}