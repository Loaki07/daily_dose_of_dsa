#include <iostream>
using namespace std;

int main() {
    int a = 10;
    // store pointer reference
    int *ptr = &a;

    float pi = 3.14;
    float *ptr2 = &pi;

    // store pointers pointer reference
    int **pptr = &ptr;
    cout << &ptr << " = " << pptr << endl;

    // deference operator
    cout << *ptr << " = " << a << endl;

    // get size of a pointer
    cout << sizeof(ptr) << endl;
    cout << sizeof(ptr2) << endl;


    cout << &pi << " = " << ptr2 << endl;
    cout << &a << " = " << ptr << endl;

    // null pointer
    int *ptr3 = NULL;
    cout << ptr3 << endl;
}