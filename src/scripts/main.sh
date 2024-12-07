#!/bin/bash

file_list() {  

    printf "\nAccess File List?[Y\N]";
    read fl;

    if [[ $fl == "y" || $fl == "Y" || $fl == "Yes" || $fl == "yes" ]]; then

        sleep 1
        echo "----------------";
        printf "  filename.txt\n";
        echo "----------------";
        cat filename.txt;
    else 
        db_log
    fi
    db_log
}

db_log() {

    # Variables
    db_word_count=$(wc -w < database.log);

    printf "\nAccess database.log?[Y\N]";
    read db;

    if [[ $db == "y" || $db == "Y" || $db == "Yes" || $db == "yes" ]]; then

        if (( db_word_count == 0 )); then
            printf "\ndatabase.log Is Empty...";
            sleep 1;
            exit 0;
        fi

        sleep 1
        echo "----------------";
        printf "  database.log\n";
        echo "----------------";
        # Adds space seperating '------------' and the contents of database.log
        printf "\n";
        cat database.log;
    else 
        dberr_log
    fi
    dberr_log
}

dberr_log() {

    # Variables
    dberr_word_count=$(wc -w < database_err.log);

    printf "\n\nAccess database_err.log?[Y\N]";
    read db_err;

    if [[  $db_err == "y" || $db_err == "Y" || $db_err == "Yes" || $db_err == "yes" ]]; then

        if (( dberr_word_count == 0 )); then  
            printf "\ndatabase_err.log Is Empty...";
            exit 0;
        fi

        sleep 1;
        echo "----------------";
        printf "database_err.log";
        echo "----------------";
        # Adds space seperating '------------' and the contents of database_err.log
        printf "\n";
        cat database_err.log;
    else 
        printf "\nExiting Soprano..."
        sleep 1
        exit 0;
    fi
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
else 
    cd "$ROOT_DIRECTORY" || { echo "Error While Trying To Move Back To Root Directory."; exit 1; };
    printf "Database Updated On: $current_date\n\n" >> database.log;
    cat "$LOCK_FILE" > filename.txt;
    printf "\nScript Executed Succesfully.\n";
    printf "\nLogs Updated.\n";
    printf "\nFiles Listed.\n";
fi

file_list

