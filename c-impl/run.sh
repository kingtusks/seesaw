#!/bin/sh

set -xe

clang -Wall -Wextra $1.c -o $1.o && ./$1.o
