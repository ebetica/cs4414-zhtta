#!/bin/sh
# OLD TESTS
#ab -n5000 -c100 http://127.0.0.1:4414/test/1char.txt > test_1.txt & 
#ab -n1000 -c10 http://127.0.0.1:4414/test/10chars.txt > test_2.txt & 
#ab -n250 -c10 http://127.0.0.1:4414/test/50chars.txt > test_3.txt & 
#ab -n100 -c5 http://127.0.0.1:4414/test/250chars.txt > test_4.txt & 
#ab -n25 -c3 http://127.0.0.1:4414/test/1000chars.txt > test_5.txt & 

if [ ! -f ./test/zhtta-test-urls.httperf ]; then
    ./test/gentestfile.sh
fi

httperf --server localhost --port 4414 --rate 60 --num-conns 60 --wlog=y,./test/zhtta-test-urls.httperf
