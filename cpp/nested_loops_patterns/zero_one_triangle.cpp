#include <iostream>
using namespace std;

void zeroOneTriangle(int n)
{
    bool val = true;

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j <= i; j++)
        {
            cout << val << " ";
            val = !val;
        }
        cout << endl;
    }
}

int main()
{
    zeroOneTriangle(5);
    return 0;
}

// 1 
// 0 1 
// 0 1 0 
// 1 0 1 0 
// 1 0 1 0 1 