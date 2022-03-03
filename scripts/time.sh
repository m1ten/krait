#!/bin/bash

# usage ./time.sh <number of iterations> <program> <arguments>

n=$1; shift
(($# > 0)) || return                 
for ((i = 0; i < n; i++)); do
	{ time -p "$@" &>/dev/null; } 2>&1 # ignore the output of the command
									
done | awk '
	/real/ { real = real + $2; nr++ }
	/user/ { user = user + $2; nu++ }
	/sys/  { sys  = sys  + $2; ns++}
	END    {
			if (nr>0) printf("real %f\n", real/nr);
			if (nu>0) printf("user %f\n", user/nu);
			if (ns>0) printf("sys %f\n",  sys/ns)
		}'