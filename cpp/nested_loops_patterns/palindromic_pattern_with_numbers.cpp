#include <iostream>
using namespace std;

void palindromicPatternWithNumbers(int n)
{
    for (int i = 1; i <= n; i++)
    {
        // spaces
        for (int j = 1; j <= n - i; j++)
        {
            cout << " ";
        }

        // nums backward
        for (int j = i; j >= 1; j--)
        {
            cout << j;
        }

        // nums forward
        for (int j = 2; j <= i; j++)
        {
            cout << j;
        }
        cout << endl;
    }
}

int main()
{
    palindromicPatternWithNumbers(5);
    return 0;
}

//     1
//    212
//   32123
//  4321234
// 543212345