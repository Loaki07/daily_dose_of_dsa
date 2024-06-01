#include <iostream>
using namespace std;

int binToDec(int binNum) {
    int n = binNum;
    int decNum = 0;
    int pow = 1; // 2^0 2^1 2^2 ...

    while (n > 0) {
        int lastDig = n % 10;
        decNum += lastDig * pow;
        pow = pow * 2;
        n = n/10;
    }

    return decNum;
}

int main() {
    cout << binToDec(101) << endl;
    cout << binToDec(1010) << endl;
    cout << binToDec(1011) << endl;
    return 0;
}