#include <iostream>
using namespace std;

void floydsTriangle(int n)
{
    int val = 1;
    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= i; j++)
        {
            cout << val++ << " ";
        }
        cout << endl;
    }
}

int main()
{
    floydsTriangle(5);
    return 0;
}

// 1
// 2 3
// 4 5 6
// 7 8 9 10
// 11 12 13 14 15