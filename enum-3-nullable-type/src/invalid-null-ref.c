#include<stdio.h>

typedef struct _A {} A;
typedef struct _B {} B;
typedef struct _C {} C;

void fn_with_optional_int(
                            int arg1,
                            int arg2,
                            int optional_arg3
                        );

void fn_with_optional_ptr(
                            A *ptr1,
                            B *ptr2,
                            C *option_ptr3
                        );

int main() {
    int x = 10;
    int y = 20;

    fn_with_optional_int(x, y, -1);

    A a = {};
    B b = {};

    fn_with_optional_ptr(&a, &b, NULL);




    return 0;
}


/*
void fn_x()
{
    int z = -1;     // "z" ideally should be +ve int.
                    // For initialization, setting a invalid value for "z";

    if(<expr>)
    {
        .....
        .....
        if(<expr>)
        {
            .....
            .....
            if(<expr>)
            {
                z = 100;
            }
            else
            {
                z = 1;
            }
        }
        else
        {
            .....
            .....
        }
    }
    else
    {
        .....
        z = 50;
    }

    if(z >= 0)
        fn_y(z);

    return;
}
*/