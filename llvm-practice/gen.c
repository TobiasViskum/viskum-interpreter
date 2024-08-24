#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

int main() {
    char temp[50];

    char myname[] = "tobias";
    myname[0] = 'T';
    myname[1] = 'O';
    myname[2] = 'B';

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