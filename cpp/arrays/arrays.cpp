#include <iostream>
using namespace std;

int main() {
    int arr[5] = {1, 2, 3, 4, 5};
    int n = sizeof(arr) / sizeof(int);

    for (int i=0; i<n; i++) {
        cin >> arr[i];
    }

    for (int i=0; i<n; i++) {
        cout << arr[i] << ",";
    }

    cout << endl;

    // find largest and min in an arr
    int arr1[] = {5, 4, 3, 9, 12};
    int len = sizeof(arr)/ sizeof(int);

    int max = arr1[0];
    int min = arr1[0];
    for (int i=0; i<len; i++) {
        if (arr1[i]> max) {
            max = arr1[i];
        }

        if (arr1[i] < min) {
            min = arr1[i];
        }
    }

    cout << max << endl;
    cout << min << endl;
    return 0;
}