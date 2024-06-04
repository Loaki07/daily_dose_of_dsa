#include <iostream>
using namespace std;

// 2 pointer approach
// O(n) time | O(1) space
void reverseArr(int arr[], int n) {
    int start = 0;
    int end = n - 1;

    while (start < end) {
        // swap
        // int temp = arr[start];
        // arr[start] = arr[end];
        // arr[end] = temp;

        // swap built-in
        swap(arr[start], arr[end]);
        start++;
        end--;
    }
}

void printArr(int arr[], int n) {
    for (int i = 0; i<n; i++) {
        cout << arr[i] << ", ";
    }
    cout << endl;
}

int main() {
    int arr[] = { 5, 4, 3, 2, 9 };
    int n = sizeof(arr) / sizeof(int);

    // two ways to reverse an arr
    // 1. with extra space
    // 2. w/o using extra space - 2 pointer approach

    reverseArr(arr, n);
    printArr(arr, n);
}