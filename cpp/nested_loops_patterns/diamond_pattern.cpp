#include <iostream>
using namespace std;

void diamondPattern(int n)
{
    // 1st pyramid
    for (int i = 1; i <= n; i++)
    {
        // spaces
        for (int j = 1; j <= n - i; j++)
        {
            cout << " ";
        }

        // stars
        for (int j = 1; j <= 2 * i - 1; j++)
        {
            cout << "*";
        }

        cout << endl;
    }

    // 2nd pyramid
    for (int i = n; i >= 1; i--)
    {
        // spaces
        for (int j = 1; j <= n - i; j++)
        {
            cout << " ";
        }

        // stars
        for (int j = 1; j <= 2 * i - 1; j++)
        {
            cout << "*";
        }

        cout << endl;
    }
}

int main()
{
    diamondPattern(7);
    return 0;
}

//       *
//      ***
//     *****
//    *******
//   *********
//  ***********
// *************
// *************
//  ***********
//   *********
//    *******
//     *****
//      ***
//       *
