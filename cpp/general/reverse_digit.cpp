#include <iostream>
using namespace std;

int reverseDigit(int n)
{
    int result = 0;

    while (n > 0)
    {
        int lastDig = n % 10;
        n /= 10;
        result = result * 10 + lastDig;
    }

    return result;
}

int main()
{
    cout << reverseDigit(10829);
    return 0;
}