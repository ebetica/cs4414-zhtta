#!/bin/sh
ab -n5000 -c100 http://127.0.0.1:4414/test/1char.txt > test_1.txt & 
ab -n1000 -c10 http://127.0.0.1:4414/test/10chars.txt > test_2.txt & 
ab -n250 -c10 http://127.0.0.1:4414/test/50chars.txt > test_3.txt & 
ab -n100 -c5 http://127.0.0.1:4414/test/250chars.txt > test_4.txt & 
ab -n25 -c3 http://127.0.0.1:4414/test/1000chars.txt > test_5.txt & 
