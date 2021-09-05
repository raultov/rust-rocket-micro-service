#!/bin/bash

# turn on bash's job control
set -m

echo "Start Cassandra"
$CASSANDRA_HOME/bin/cassandra -R

echo "Sleep 20 seconds"
sleep 20

echo "Setup Cassandra data"
cqlsh -f 'IT/cassandra-setup.cql'

echo "Build rust app"
cargo build --release

echo "Launch rust app"
target/release/rust_rocket_micro_service &

echo "Sleep 5 seconds"
sleep 5

echo "Executing integration tests"
for entry in IT/tests/*.sh
do
    bash "$entry"
    retn_code=$?

    if [ "$retn_code" != 0 ]
    then
        exit $retn_code
    fi
done

exit 0
