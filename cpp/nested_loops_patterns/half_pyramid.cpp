#include <iostream>
using namespace std;

void halfPyramid(int n)
{
    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= i; j++)
        {
            cout << j;
        }
        cout << endl;
    }
}

int main()
{
    halfPyramid(5);
    return 0;
}

// 1
// 12
// 123
// 1234
// 12345