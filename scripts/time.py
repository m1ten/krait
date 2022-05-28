#!bin/python3

# usage: python3 time.py <number of iterations> <program> <arguments>

# get the variables from the command line
import sys
iterations = int(sys.argv[1])
program = sys.argv[2]
arguments = sys.argv[3:]


# run a program for a number of iterations with the arguments and print the time taken
import time
import subprocess
start = time.time()
for i in range(iterations):
	subprocess.call([program] + arguments)
end = time.time()
print(end - start)