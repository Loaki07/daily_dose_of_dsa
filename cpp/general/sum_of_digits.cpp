#include <iostream>
using namespace std;

int sumOfDigits(int n)
{
    int res = 0;
    // to get the last digit do modulo 10
    // to remove the last digit divide by 10
    while (n > 0)
    {
        int lastDig = n % 10;
        n = n / 10;
        res += lastDig;
    }

    return res;
}

int main()
{
    cout << sumOfDigits(10829);
    return 0;
}