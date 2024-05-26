#include <iostream>
#include <cmath>
using namespace std;

bool isPrime(int n)
{
    if (n <= 1)
    {
        return false;
    }

    for (int i = 2; i <= sqrt(n); i++)
    {
        // i is a factor of n;
        // i completely divides n;
        // n is non-prime
        if (n % i == 0)
        {
            return false;
        }
    }

    return true;
}

int main()
{
    // Test cases
    int test1 = 15;
    int test2 = 29;
    int test3 = 7;

    cout << "Is " << test1 << " a prime number? " << (isPrime(test1) ? "Yes" : "No") << endl;
    cout << "Is " << test2 << " a prime number? " << (isPrime(test2) ? "Yes" : "No") << endl;
    cout << "Is " << test3 << " a prime number? " << (isPrime(test3) ? "Yes" : "No") << endl;

    return 0;
}