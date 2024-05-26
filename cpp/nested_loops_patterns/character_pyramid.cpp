#include <iostream>
using namespace std;

void characterPyramid(int n)
{
    char ch = 'A';

    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= i; j++)
        {
            cout << ch++;
        }
        cout << endl;
    }
}

int main()
{
    characterPyramid(5);
    return 0;
}

// A
// BC
// DEF
// GHIJ
// KLMNO
