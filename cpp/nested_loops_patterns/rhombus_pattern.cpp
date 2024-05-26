#include <iostream>
using namespace std;

void rhombusPattern(int n)
{
    for (int i = 0; i < n; i++)
    {
        // spaces
        for (int j = 0; j <= n - 1 - i; j++)
        {
            cout << " ";
        }

        // stars
        for (int j = 0; j < n; j++)
        {
            cout << "*";
        }

        cout << endl;
    }
}

int main()
{
    rhombusPattern(5);
    return 0;
}

//      *****
//     *****
//    *****
//   *****
//  *****