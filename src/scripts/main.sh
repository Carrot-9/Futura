#!/bin/bash

# alert.sh will be run once a week via Windows Task Scheduler 
# If main.rs returns an error, the script sends an error message to database.log and exits unsucessfully 
# If main.rs runs smoothly, all new .wav files are moved to the table 'songs' in sopranodb

# Variables 

LOCK_FILE=$(mktemp /tmp/lock.XXXXXX);
trap 'rm -f "$LOCK_FILE"' EXIT

current_date=$(date +%F_%T);

# List .wav files and moves it into database.log

printf "$current_date\n\n" >> file_names.txt
ls $DIR_PATH >> file_names.txt

# Entering $LOCK_FILE... 

{

# Executes Rust Binary

cd ../../../target/release;

./soprano.exe

EOF
} > "$LOCK_FILE" > /dev/null 2>&1

# Exiting $LOCK_FILE...

# Checks Exit Code

if [[ $? -ne 0 ]]; then 
    printf "$current_date\n" >> database.log
    printf "No new .wav files in table 'songs'\n" >> database.log;
    exit 0;
else 
    printf "$current_date\n" >> database.log
    printf "New .wav file added to table 'songs'\n" >> database.log;
    exit 0;
fi




