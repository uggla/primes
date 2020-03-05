#!/usr/bin/env bash

for pin in $(seq -w 0000 9999); do
  md5=$(echo "${pin}" | md5sum)
  echo "pin:${pin}|hash:${md5}" >>code_hash.txt
done
