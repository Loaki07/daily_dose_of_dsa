#include <iostream>
using namespace std;

int factorial(int n)
{
    int fact = 1;
    for (int i = 1; i <= n; i++)
    {
        fact *= i;
    }

    return fact;
}

int binomialCoefficient(int n, int r)
{
    int val1 = factorial(n);
    int val2 = factorial(r);
    int val3 = factorial(n - r);

    int result = val1 / (val2 * val3);
    return result;
}

int main()
{
    int result = binomialCoefficient(4, 2);
    cout << result << endl;
    int res = binomialCoefficient(3, 2);
    cout << res << endl;
    return 0;
}