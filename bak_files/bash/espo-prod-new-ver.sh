#!/bin/bash

#update-switch-esposite-prod.sh

echo "***Now on Rust***"
echo "/usr/home/ancnet1/rs_bak_prod/bak_files/bash/espo-prod-new-ver.sh"
# Run manually via SSH. This will use the espo PHP command to run the update.

# NOTE: We now use the -d parm for the memory_limit. We could use the -c parm
# to call in the .user.ini file.
# php -c /usr/home/ancnet1/public_html/anc123.com/espocrm2/.user.ini command.php upgrade
 

cd /usr/home/ancnet1/public_html/anc123.com/espocrm2



read -p "Do you want to update the espo prod via PHP84 to new version (yes/no):  " var3

if [ $var3 == "yes" ]
then
/usr/local/bin/php84 -d  memory_limit=524M command.php upgrade
echo "---done with prod"
fi

echo " "

read -p "Do you want to upgrade the BAK file via PHP84 to new version (yes/no):  " var1

if [ $var1 == "yes" ]
then
# go to the backup site directory
cd /usr/home/ancnet1/public_html/anc77.pairsite.com/espocrm2
/usr/local/bin/php84 -d  memory_limit=524M command.php upgrade
echo "---done with BAK"

fi

echo " "
echo "If you want to move Espo to Rsync choose 'esposite' in the Switch3-rust procedure "

echo "(https://anc123.com/switch3/index.php). "
echo " "


#read -p "Do you want to set the switch-file.txt file to send to rsync.net (yes/no):  " var2

#if [ $var2 == "yes" ]
#then

#cd /usr/home/ancnet1/bak-scripts
#source set-switch-file-txt.sh

#fi

# 5-31-2023 

#***** We do this here, not previously in espo-version.sh
#***** We do this here, not previously in espo-version.sh
#***** We do this here, not previously in espo-version.sh

read -p "Do you want to set 'previous file' to  'current file' (*yes* or no):  " var3

if [ $var3 == "yes" ]
then

   # Change the previous file to equal the current version file

   # Get the current version from espo site and put in a 'from espo' file
   
    curl -s https://www.espocrm.com/download/ |   grep  'Latest Release EspoCRM' > /usr/home/ancnet1/rs_bak_prod/bak_files/prev-espo-ver.txt

  #sftp://ancnet1.pairserver.com//usr/home/ancnet1/rs_bak_prod/bak_files/bash/espo-prod-new-ver.sh 

   # Now put the 'from espo' file text into a var_from_espocrm so we can transfer it

   #var_from_espocrm2=$( cat /usr/home/ancnet1/py-backup/bak-files/from_espocrm.txt)


   # Now pop it in the 'prev version' file. This could have been done quicker 
   # but code was mostly there already. 

   #echo "$var_from_espocrm2" > /usr/home/ancnet1/py-backup/bak-files/prev_espocrm.txt

# for rust

   #echo "$var_from_espocrm2" > /usr/home/ancnet1/rs_bak_prod/bak_files/prev-espo-ver.txt

fi



echo "---Prev Espo is now set---"

echo "---All done with espocrm update and backup---"
echo "---Remember to do manual backup via switch3 procedure---"




