#! /bin/bash
echo "Starting " |  tee "$0".log;
while true
do
  date | tee -a $0.log;
  cargo run 2>&1 | tee -a $0.log;
  sleep 300;
done
