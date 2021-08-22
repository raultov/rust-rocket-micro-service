#!/bin/bash

echo "2-get-vehicle-performance.sh"

response=$( ab -d -t 30 -c 1000 -n 500000 'http://localhost:8000/api/vehicle/d13fe953-297a-4781-807a-f9becc1b71f6/60e18f00-34b8-4a52-916c-adbb0204618e' )
req_per_sec=$(( $(echo $response | sed 's/.*Requests per second: \([0-9]\+\).*/\1/') ))
failed_requests=$(( $(echo $response | sed 's/.*Failed requests: \([0-9]\+\).*/\1/') ))

echo "Request per second: $req_per_sec"
echo "Num failed requests: $failed_requests"

# 12000 requests/second is a calculated threshold, really dependant on the specific hardware where this script is executed
if [ "$req_per_sec" -lt 12000 ] || [ "$failed_requests" -gt 1 ]
then
    echo "Test failed! Either Less than 12000 request/seconds processed or more than 1 failed request"
    exit 1
fi

exit 0
