#include <iostream>
using namespace std;

// pass by reference using pointers
void changeA(int *ptr) {
    *ptr = 20;
}

// pass by reference using reference valriables
void changeB(int &b) {
    b = 20;
}

int main() {
    int a = 10;
    changeA(&a);

    cout << a << endl;

    int b = 10;
    changeB(b);
    cout << b << endl;

    // pointers practice Q
    int x = 5, y = 10;
    int *ptr = &x, *ptr2 = &y;
    ptr2 = ptr;

    // all should now point to the
    // same memory location
    cout << ptr2 << endl;
    cout << ptr << endl;
    cout << &x << endl;

    // this is same as initiliazing a null pointer
    int *ptr3 = 0;
    cout << ptr3 << endl;

    return 0;
}