#!/bin/bash

# main.sh will run daily at 12:00 PM

# Loads .env variables
source "$ENV" > /dev/null 2>&1;

# Variables 
LOCK_FILE=$(mktemp /tmp/lock.XXXXXX);
trap 'rm -f "$LOCK_FILE"' EXIT

current_date=$(date +%F_%T);

# Moves to root directory /soprano so alias can execute outside this directory
cd "$ROOT_DIRECTORY" || { echo "Error While Trying To Move To Root Directory."; exit 1; };

# Moves to location of rust binary
cd "$BINARY_PATH" || { echo "Error While Trying To Move To Location Of Binary."; exit 1; };

# Executes Rust Binary and moves stdout and stderr to LOCK_FILE
./soprano.exe > "$LOCK_FILE";

if [[ $? -ne 0 ]]; then
    cd "$ROOT_DIRECTORY" || { echo "Error While Trying To Move Back To Root Directory."; exit 1; };
    printf "$current_date\n" >> database_err.log && printf "ERROR!:\n" $LOCK_FILE  2>> database_err.log;
    printf "\nScript Did Not Execute Succesfully.\n";
else 
    cd "$ROOT_DIRECTORY" || { echo "Error While Trying To Move Back To Root Directory."; exit 1; };
    printf "Database Updated On: $current_date\n" >> database.log;
    printf "\nScript Executed Succesfully.\n";
fi