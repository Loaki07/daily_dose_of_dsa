#include <iostream>
using namespace std;

// single int value -> 4 bytes
// single ch value -> 1 bytes


void printArr(int *ptr, int n) {
    for (int i=0; i<n; i++) {
        cout << (*ptr + i) << endl;
    }
}

// pointer airthmetic
// ptr1 + ptr2 is invalid
// ptr1 - ptr2 is valid if both is of the same type

int main() {
    int a = 10;
    int *aptr = &a;

    cout << aptr << endl;
    aptr++; // moves the pointer ahead by 4 bytes
    cout << aptr << endl;
    aptr--;
    cout << aptr << endl; // - 4 bytes
    cout << (aptr + 3) << endl; // + 12 bytes

    char ch = 'a';
    char *chptr = &ch;

    cout << chptr << endl;
    chptr++; // moves the pointer ahead by 1 byte
    cout << chptr << endl;

    int arr[] = {1, 2, 3, 4, 5};
    int n = sizeof(arr) / sizeof(int);
    printArr(arr, n);
    
    int b = 5;
    int *ptr1 = &a;
    int *ptr2 = ptr1 + 3;

    cout << ptr2 << endl;
    cout << ptr1 << endl;

    cout << ptr2 - ptr1 << endl;

    // Pointer comparison
    cout << (ptr2 > ptr1) << endl; // yes: true : 1
    cout << (ptr2 == ptr1) << endl; // no: false : 0
    return 0;
}