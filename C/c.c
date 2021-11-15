// build => clang program.c -o program
// Generate assembly => gcc -S fileName.c

#include <stdio.h>
#include <stdlib.h> // For the atoi() function

char f[4] = "Fizz";
char b[4] = "Buzz";
char fb[8] = "FizzBuzz";

int main(int argc, char *argv[])
{
    // char s[2];
    // fgets(s,2,stdin);
    int n = atoi(argv[1]);
    // int next = n + 1;
    for (int i = 1; i < n + 1; i++)
    {
        if ((i % 3 == 0) && (i % 5 == 0))
        {
            fputs(fb, stdout);
        }
        else if (i % 3 == 0)
        {
            fputs(f, stdout);
        }
        else if (i % 5 == 0)
        {
            fputs(b, stdout);
        }
        else
        {
            fprintf(stdout, "%d", i);
        }
    }
}