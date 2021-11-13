#!/bin/bash

echo "4-create-vehicle-performance.sh"

response=$( ab -p create_vehicle.json -T "application/json" -d -t 30 -c 1000 -n 20000 "http://localhost:8000/api/vehicle" )
reqs_per_sec=$(( $(echo $response | sed 's/.*Requests per second: \([0-9]\+\).*/\1/') ))
failed_requests=$(( $(echo $response | sed 's/.*Failed requests: \([0-9]\+\).*/\1/') ))

echo "Requests per second: $reqs_per_sec"
echo "Num failed requests: $failed_requests"

# 3000 requests/second is a pre-calculated threshold, really dependant on the specific hardware where this script is executed
if [ "$reqs_per_sec" -lt 3000 ] || [ "$failed_requests" -gt 1 ]
then
    echo "Test failed! Either Less than 3000 requests/second processed or more than 1 failed request"
    exit 1
fi

exit 0
