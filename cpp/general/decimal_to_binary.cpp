#include <iostream>
using namespace std;

int decToBin(int dec) {
    int n = dec;
    int pow = 1; // 10^0 10^1 10^2 ...
    int binNum = 0;

    while (n > 0) {
        int rem = n % 2;
        binNum += rem * pow;
        n = n/2;
        pow = pow * 10;
    }

    return binNum;
}

int main() {
    cout << decToBin(5) << endl;
    cout << decToBin(4) << endl;
    cout << decToBin(10) << endl;
    return 0;
}