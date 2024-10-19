#!/bin/bash

# Runs main.rs when new .wav file is added to Downloads Folder 

LIST=$(mktemp); ls $DIR_PATH > $LIST; cat $LIST;




