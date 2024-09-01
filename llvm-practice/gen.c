#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

int add(int x, int y) {
    return x + y;
}

int main() {
    char temp[50];

    int a = 1;
    int b = 9;

    int items[] = {a+b, 20, 30, 40, 50};

    int items2[5];
  

    items2[0] = 10;
    items2[1] = 20;
    items2[2] = 30;
    items2[3] = 40;
    items2[4] = 40;
    printf("U runsdfdsf");



    char myname[] = "tobias";
    myname[0] = 'T';
    myname[1] = 'O';
    myname[2] = 'B';

    printf("Hello word\n");

    char name1[] = "Tobias";
    char name2[] = "Andreas";
    int iseq = strcmp(name2, name1);
    printf("Is eq: %d\n", iseq);

    printf("%s", myname);

    char* hello = "Hello";
    char* anotherhello = "Hello";
    char* unkown_string = ""; // This is not known at compile time
    char* world = "World!";
    
    sprintf(temp, "%s%s%s", hello, unkown_string, world);
    int len = strlen(temp);
    char* final = (char*)malloc((len + 1) * sizeof(char));
    strcpy(final, temp);
    printf("Concatenated String: %d%s\n", len, final);
    return 0;
}