#!/bin/sh

# Automatically deletes table 'songs' from sopranodb

mysql -D sopranodb -u root -p -e 'DROP TABLE songs;'