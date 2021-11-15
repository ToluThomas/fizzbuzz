#!/usr/bin/env python3

import sys

j = sys.argv[1]
i = 1
f = 'Fizz'
b = 'Buzz'
fb = 'FizzBuzz'

while (i < int(j) + 1) :
    if (i % 3 == 0 & i % 5 == 0):
        print(fb)
    elif (i % 3 == 0):
        print(f)
    elif (i % 5 == 0):
        print(b)
    else :
        print(str(i))
    i += 1

