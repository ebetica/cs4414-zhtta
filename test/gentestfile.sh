#!/bin/bash

dd if=/dev/urandom of=5K.bin bs=5K count=1
dd if=/dev/urandom of=5M.bin bs=5M count=1
dd if=/dev/urandom of=10M.bin bs=10M count=1
dd if=/dev/urandom of=20M.bin bs=20M count=1
dd if=/dev/urandom of=40M.bin bs=40M count=1
dd if=/dev/urandom of=80M.bin bs=80M count=1
dd if=/dev/urandom of=512M.bin bs=512M count=1

wget http://www.cs.virginia.edu/~wx4ed/cs4414/ps3/zhtta-test-urls.txt
awk '{print "/test" $0;}' zhtta-test-urls.txt | tr "\n" "\0" > zhtta-test-urls.httperf
rm zhtta-test-urls.txt
