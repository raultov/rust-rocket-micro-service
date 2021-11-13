#!/bin/bash

echo "3-create-vehicle.sh"

vehicle_body=$(cat <<-END
  {
    "name": "test create vehicle",
    "user_id": "96587b88-9e56-479f-972e-c1f4c26d41b6",
    "created_at": "2018-12-02T00:00:00.111Z",
    "vehicle_type": "bike",
    "retired_at": "2018-12-02T00:00:00.111Z",
    "brand": "Orbea",
    "model": "chunga",
    "distance": 400,
    "owner_since": "2015-12-02",
    "manufacturing_date": "2015-12-02",
    "picture": "a picture.jpg"
  }
END
)

response=$( curl -s -X POST "http://localhost:8000/api/vehicle" -H "Content-Type: application/json" -d "${vehicle_body}" )
name=$( jq -r  '.name' <<< "${response}" )

if [ "$name" != "test create vehicle" ]
then
    echo "Test failed! name does not contain the expected value"
    exit 1
fi

exit 0
