#!/usr/bin/env python3

from datetime import datetime, timedelta
from time import sleep

CHECK_INTERVAL = 15
CHECKS = 20
MIN_DELTA = timedelta(seconds=5).total_seconds()

time_arr = []
deltas = []

for i in range(CHECKS):
    time = datetime.utcnow()
    print('Reported time:', time)
    time_arr.append(time)
    if i > 0:
        deltas.append((time_arr[i] - time_arr[i - 1]).total_seconds())

    sleep(CHECK_INTERVAL)

max_delta = -min(deltas)
print('deltas: ', deltas)
print('max negative delta:', max_delta)

if max_delta <= MIN_DELTA:
    raise Exception('Time did not move backwards during the test')
