#include <iostream>
using namespace std;

void startPattern(int n)
{
    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= i; j++)
        {
            cout << "* ";
        }

        cout << endl;
    }
}

int main()
{
    startPattern(5);
    return 0;
}

// *
// * *
// * * *
// * * * *
// * * * * *