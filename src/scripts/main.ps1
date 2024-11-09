# Windows Task Scheduler runs main.sh daily at 09:00
schtasks /create /tn "run_soprano" /tr "" /sc daily /st 09:00 
