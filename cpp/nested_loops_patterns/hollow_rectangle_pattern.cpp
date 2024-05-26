#include <iostream>
using namespace std;

void hollowRectangle(int n)
{
    for (int i = 1; i <= n; i++)
    {
        cout << "*"; // first

        for (int j = 1; j <= n - 1; j++)
        {
            if (i == 1 || i == n)
            {
                cout << "*";
            }
            else
            {
                cout << " ";
            }
        }

        cout << "*"; // last
        cout << endl;
    }
}

int main()
{
    hollowRectangle(5);
    return 0;
}

// ******
// *    *
// *    *
// *    *
// ******