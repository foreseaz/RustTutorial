#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>
#include <assert.h>
#include "../fruit.h"

int test_callback(char *msg)
{
    printf("*********************callback %s\n", msg);
    printf("#########################\n");
    return 0;
}

int test1()
{
    Fruit *f = (Fruit *)malloc(sizeof(Fruit));
    f->price = 200;
    //f->call_back = &test_callback;
    set_callback(f, &test_callback);
    while (true)
    {
        display(f);
        sleep(1);
        f->price++;
    }

    free(f);
}
int main()
{
    char tmp[200];
    int actual = add_text("apple", tmp, 200);
    printf("actual %d\n", actual);
    printf("%s\n", tmp);

    return 0;
}
