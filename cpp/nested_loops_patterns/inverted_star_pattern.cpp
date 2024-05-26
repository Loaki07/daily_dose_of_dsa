#include <iostream>
using namespace std;

void invertedStarPattern(int n)
{
    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= (n - i + 1); j++)
        {
            cout << "* ";
        }
        cout << endl;
    }
}

int main()
{
    invertedStarPattern(5);
    return 0;
}

// * * * * *
// * * * *
// * * *
// * *
// *