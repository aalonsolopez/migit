#!/bin/bash

if [ "$(pwd)/.migit" ]; then
    rm -r "$(pwd)/.migit"
    echo ".migit file removed."
else
    echo ".migit file does not exist."
fi