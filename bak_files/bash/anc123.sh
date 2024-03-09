#!/bin/bash

exit

# THIS IS NOT USED. CODE IS IN RUST INSTEAD
# THIS IS NOT USED. CODE IS IN RUST INSTEAD

# THIS IS NOT USED. CODE IS IN RUST INSTEAD


#dte=$1

#fn1="/usr/home/ancnet1/rs_bak_prod/bak_files/anc123.com-rs-"$dte".zip"








fn1=$1
zip -rq  $fn1  /usr/home/ancnet1/public_html/anc123.com   -x "/usr/home/ancnet1/public_html/anc123.com/baikal94a/*"       "/usr/home/ancnet1/public_html/anc123.com/espocrm2/*" "/usr/home/ancnet1/public_html/anc123.com/piwigopix/*"    "/usr/home/ancnet1/public_html/anc123.com/shop/*"   "/usr/home/ancnet1/public_html/anc123.com/urls9296/*"





# delete large sql file
#rm -f $fn2

# get rid of previous .gz at datacenter
#ssh  fm1364@fm1364.rsync.net rm  espo-db-backup-rs/*.gz

#send it up to the datacenter
#rsync -rqp     $fn  fm1364@fm1364.rsync.net:espo-db-backup-rs/ 


