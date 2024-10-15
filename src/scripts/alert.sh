#!/bin/bash

# Runs main.rs when new .wav file is added to Downloads Folder 

exec 5> debug_output.txt
BASH_XTRACEFD="5"
set -x

LIST=$(mktemp);

ls $DIR_PATH > $LIST 




