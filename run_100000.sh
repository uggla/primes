#!/usr/bin/env bash

time cargo test --test sum_primes_100000 -- --nocapture
