#!/bin/bash

echo "1-get-vehicle.sh"

response=$( curl -s -X GET 'http://localhost:8000/api/vehicle/d13fe953-297a-4781-807a-f9becc1b71f6/60e18f00-34b8-4a52-916c-adbb0204618e' )
name=$( jq -r  '.name' <<< "${response}" )
vehicle_id=$( jq -r  '.vehicle_id' <<< "${response}" )

if [ "$name" != "test vehicle 2" ] || [ "$vehicle_id" != "60e18f00-34b8-4a52-916c-adbb0204618e" ]
then
    echo "Test failed! Either name or vehicle_id does not contain the expected value"
    exit 1
fi

exit 0
