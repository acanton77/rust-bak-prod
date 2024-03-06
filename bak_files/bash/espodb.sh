#!/bin/bash

dte=$1

fn1="/usr/home/ancnet1/rs_bak_prod/bak_files/espo-db-backup-dir/espo-db-prod-"$dte".gz"
fn2="/usr/home/ancnet1/rs_bak_prod/bak_files/espo-db-backup-dir/espo-db-prod-"$dte".sql"

# dump to zip for datacenter
/usr/local/bin/mysqldump --login-path=espocrm   -eqf  --add-drop-table  ancnet1_espo1   | gzip > $fn1

#dump to sql for update of backup
/usr/local/bin/mysqldump --login-path=espocrm   -eqf  --add-drop-table  ancnet1_espo1   > $fn2

# update backup 
/usr/local/bin/mysql --login-path=espocrmbak ancnet1_espobak < $fn2


# delete large sql file
rm -f $fn2

# get rid of previous .gz at datacenter
#ssh  fm1364@fm1364.rsync.net rm  espo-db-backup-rs/*.gz

#send it up to the datacenter
#rsync -rqp     $fn  fm1364@fm1364.rsync.net:espo-db-backup-rs/ 








