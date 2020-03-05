#!/usr/bin/env bash

hyperfine -w 5 -M 20 "target/debug/sum_primes-08eb8b7ea8291940" "target/debug/sum_primes_100-9dcfa759cc5137ac"
