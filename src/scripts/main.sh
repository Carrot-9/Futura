#!/bin/bash

# alert.sh will be run once a week via Windows Task Scheduler 
# If main.rs returns an error, the script sends an error message to database.log and exits unsucessfully 
# If main.rs runs smoothly, all new .wav files are moved to the table 'songs' in sopranodb

# Loads .env variables
source $ENV > /dev/null 2>&1;

# Variables 
LOCK_FILE=$(mktemp /tmp/lock.XXXXXX);
trap 'rm -f "$LOCK_FILE"' EXIT

current_date=$(date +%F_%T);

# Moves to root directory /soprano so alias can execute outside this directory
cd $ROOT_DIRECTORY;

# Moves to location of rust binary
cd $BINARY_PATH;

{

./soprano.exe

} > "$LOCK_FILE" > /dev/null 2>&1;

# Sends to a log to be checked
if [[ $? -ne 0 ]]; then 
    cd $ROOT_DIRECTORY;
    printf "$current_date\n" >> database.log
    printf "No new .wav files in table 'songs'\n" >> database.log;
else 
    cd $ROOT_DIRECTORY;
    printf "$current_date\n" >> database.log
    printf "New .wav file added to table 'songs'\n" >> database.log;
fi

# Indicate Success
printf "\nScript Executed Succesfully.\n";




