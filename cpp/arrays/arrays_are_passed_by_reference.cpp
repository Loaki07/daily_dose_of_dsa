#include <iostream>
using namespace std;

void func(int arr[]) {
    arr[0] = 1000;
}

void printArr(int arr[], int n) {
    for (int i = 0; i<n; i++) {
        cout << arr[i] << ", ";
    }
    cout << endl;
}

int main() {
    int arr[] = {1, 2, 3, 4, 5};
    int n = sizeof(arr) / sizeof(int);

    // passing array name is eq. to passing the pointer
    func(arr);
    cout << arr[0] << endl;

    // the len of array needs to be passed
    // since in c++ you cannot calculate the len when
    // passed as reference
    printArr(arr, n);
    return 0;
}