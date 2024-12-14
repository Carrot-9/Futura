#!/bin/bash

home() {
    printf "\n\nOptions List:\n\n";
    printf "Access File List?(0)\n";
    printf "Access Database.log?(1)\n";
    printf "Access Database_err.log?(2)\n";
    printf "Exit Soprano?(3)\n";
    read options;

    case $options in 
        "0")
            file_list;;
        "1")
            db_log;;
        "2") 
            dberr_log;;
        "3")
            Exit;;
        esac
            
}

file_list() {  

        sleep 1
        echo "----------------";
        printf "  filename.txt\n";
        echo "----------------";
        cat filename.txt;

        home
}

db_log() {

    # Variables
    db_word_count=$(wc -w < database.log);

        if (( db_word_count == 0 )); then
            printf "\ndatabase.log Is Empty...Exiting Soprano.";
            sleep 1
            exit 0;
        fi

        sleep 1
        echo "----------------";
        printf "  database.log\n";
        echo "----------------";
        # Adds space seperating '------------' and the contents of database.log
        printf "\n";
        cat database.log;

        home
}

dberr_log() {

    # Variables
    dberr_word_count=$(wc -w < database_err.log);

        if (( dberr_word_count == 0 )); then  
            printf "\ndatabase_err.log Is Empty...Exiting Soprano.";
            sleep 1 
            exit 0;
        fi

        sleep 1
        echo "----------------";
        printf "database_err.log\n";
        echo "----------------";
        # Adds space seperating '------------' and the contents of database_err.log
        printf "\n";
        cat database_err.log;

        home
}

Exit() {
    printf "Exiting Soprano...";
    sleep 1 
    return 0;
}

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
    printf "$current_date\n" >> database_err.log && printf "ERROR!:\n\n $LOCK_FILE" 2>> database_err.log;
    printf "\nScript Did Not Execute Succesfully.\n";
    exit 1;
else 
    cd "$ROOT_DIRECTORY" || { echo "Error While Trying To Move Back To Root Directory."; exit 1; };
    printf "Database Updated On: $current_date\n\n" >> database.log;
    cat "$LOCK_FILE" > filename.txt;
    printf "\nScript Executed Succesfully.\n";
    printf "\nLogs Updated.\n";
    printf "\nFiles Listed.\n";
fi

home

