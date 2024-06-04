#include <iostream>
using namespace std;

// O(n) time | O(1) space
int linearSearch(int *arr, int n, int key) {
    for (int i=0; i<n; i++) {
        if (arr[i] == key) {
            return i;
        }
    }
    return -1;
}

int main() {
    int arr[] = { 2, 4, 6, 8, 10, 12};
    int n = sizeof(arr) / sizeof(int);

    int res = linearSearch(arr, n, 10);
    cout << res << endl;
}