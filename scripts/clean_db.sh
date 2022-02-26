#!/bin/bash

PREFIX='test'
export PGPASSWORD=password
export PGUSER=postgres
export PGHOST=localhost
export PGPORT=5434

TEST_DB_LIST=$(psql -l | awk '{ print $1 }' | grep -v template | grep -v postgres)
echo $TEST_DB_LIST
for TEST_DB in $TEST_DB_LIST ; do
    echo $TEST_DB
    if [[ $(echo $TEST_DB | grep $PREFIX) == $TEST_DB ]]
    then
        echo "Dropping $TEST_DB"
        dropdb --if-exists $TEST_DB
    fi
done