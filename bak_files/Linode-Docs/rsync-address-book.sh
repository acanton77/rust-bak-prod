

#Empty the log files
#
#exit

#RUN AT 12:01 AM EACH DAY
#RUN AT 12:01 AM EACH DAY


> /Users/al/documents/wget/rsync-address-log.txt
> /Users/al/documents/wget/rsync-address-vcf-log.txt
> /Users/al/documents/wget/rsync-documents-log.txt
> /Users/al/documents/wget/rsync-desktopstuff-log.txt
> /Users/al/documents/wget/rsync-music-log.txt
> /Users/al/documents/wget/rsync-pictures-log.txt
> /Users/al/documents/wget/rsync-address-owc-log.txt
#
echo "==============" >> /Users/al/documents/wget/rsync-address-log.txt
date >> /Users/al/documents/wget/rsync-address-log.txt
echo "==============" >> /Users/al/documents/wget/rsync-address-log.txt

echo "==============" >> /Users/al/documents/wget/rsync-address-vcf-log.txt
date >> /Users/al/documents/wget/rsync-address-vcf-log.txt
echo "==============" >> /Users/al/documents/wget/rsync-address-vcf-log.txt
#
#

# touch ~/desktop/zzzz-test-time-addressbook.txt





#
# Create a .vcf file just in case. Done from Apple Script that put it in Backups/address-book
rm  /Users/al/Backups/address-book-vcf/*.vcf >/dev/null 2>&1

#echo "before vcf script"

# open -W /users/al/documents/wget/make_vcf_file_333.app
open -W /users/al/documents/wget/make_vcf_file_444.app

#echo "after vcf script"




echo "=== Made address vcf file ==" >> /Users/al/documents/wget/rsync-address-vcf-log.txt
#

bdate=$(date  +"%m%d%y%H%M%S")



# mv /Users/al/Backups/address-book-vcf/contacts-all-2.vcf /Users/al/Backups/address-book-vcf/contacts-all-2-$bdate.vcf

#echo "before mv"


mv /Users/al/downloads/contacts-all-444.vcf /Users/al/Backups/address-book-vcf/contacts-all-2-$bdate.vcf


#echo "after mv"


 

#Send the VCF file up to server

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_rsa' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" adamsnet@adams-blake.net:/usr/home/adamsnet/backup-address-book


### prev:
# rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_rsa' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" anc1@192.53.126.91:/home/anc1/address-book-backup-dir

#Now using new 'ed25519' secure keys

# rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" anc1@74.207.240.199:/home/anc1/address-book-backup-dir


#For Pair server
#send vcf file to server

#echo "before upload to pair"


#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/bak-scripts/address-book-backup-dir

#Rsync to Python

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/py-backup/bak-files/address-book-backup-dir


#Rsync to PAIR Rust

rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-vcf-log.txt "/Users/al/Backups/address-book-vcf" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs






#echo "after upload to pair"


# We also do the entire folder. Only does incremental so won't take long.

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_rsa' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Library/Application Support/AddressBook" adamsnet@adams-blake.net:/usr/home/adamsnet/backup-address-book
#

#Also send full address book to pair serverinfo

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Library/Application Support/AddressBook" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/bak-scripts/address-book-backup-dir
#


# BusyContacts address book... whole folder.
#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_rsa' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Backups/BusyContacts" adamsnet@adams-blake.net:/usr/home/adamsnet/backup-address-book
#

touch /Users/al/Backups/BusyContacts/busy-contact-update-time.txt

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Backups/BusyContacts" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/bak-scripts/address-book-backup-dir
#

## SEND TO PYTHON DIR

#rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Backups/BusyContacts" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/py-backup/bak-files/address-book-backup-dir



# send Busy Contacts to Rust folder too

rsync -aPzq -e 'ssh -i /Users/al/.ssh/id_ed25519' --delete --log-file=/Users/al/documents/wget/rsync-address-log.txt "/Users/al/Backups/BusyContacts" ancnet1@ancnet1.pairserver.com:/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs




#/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir








# We touch the remote file date and LOCAL file date for time.
#ssh adamsnet@adams-blake.net 'touch /usr/home/adamsnet/backup-address-book/Address-Book-time.txt'

#ssh -i /Users/al/.ssh/id_ed25519 ancnet1@ancnet1.pairserver.com touch /usr/home/ancnet1/bak-scripts/address-book-backup-dir/BusyContacts-address-book-update-time.txt  /usr/home/ancnet1/bak-scripts/address-book-backup-dir



#ssh -i /Users/al/.ssh/id_ed25519 ancnet1@ancnet1.pairserver.com touch /usr/home/ancnet1/py-backup/bak-files/address-book-backup-dir/BusyContacts-address-book-update-time.txt  


#touch /Users/al/Backups/BusyContacts/busy-contact-update-time.txt

# TOUCH RUST BUSY CONTACTS TIME
ssh -i /Users/al/.ssh/id_ed25519 ancnet1@ancnet1.pairserver.com touch /usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs/BusyContacts-address-book-update-time.txt 

# WAS FOR PYTHON
#/usr/home/ancnet1/py-backup/bak-files/address-book-backup-dir/BusyContacts-address-book-update-time.txt  








#
#
#CHECK TO SEE IF LOCAL DEVICE IS CONNECTED. IF NOT, WRITE LOG ONLY
#
if [ -e /volumes/OWC-1 ]; 
then 
echo "======LOCAL=VCF=======" >> /Users/al/documents/wget/rsync-address-owc-log.txt
echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-address-owc-log.txt
echo "=========================" >> /Users/al/documents/wget/rsync-address-owc-log.txt

#
#
rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-address-owc-log.txt "/Users/al/Backups/address-book-vcf" /volumes/owc-1/backup-address-book-vcf 2> /dev/null
#
touch /volumes/owc-1/backup-address-book-vcf/last-update.txt  /Users/al/Backups/address-book-time.txt
#
#
# Back up Documents to OWC
#
echo "====LOCAL=Documents========" >> /Users/al/documents/wget/rsync-documents-log.txt
echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-documents-log.txt
echo "=========================" >> /Users/al/documents/wget/rsync-documents-log.txt
#
rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-documents-log.txt /Users/al/Documents /volumes/owc-1/backup-documents 
#
touch /volumes/owc-1/backup-documents/last-update.txt /volumes/owc-1/backup-documents
#

# Back up DesktopStuff to OWC
#
echo "======LOCAL=Desktopstuff==================" >> /Users/al/documents/wget/rsync-desktopstuff-log.txt
echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-desktopstuff-log.txt
echo "=========================" >> /Users/al/documents/wget/rsync-desktopstuff-log.txt
#
rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-desktopstuff-log.txt /Users/al/Desktopstuff /volumes/owc-1/backup-desktopstuff 

rsync -arPq  --delete  /Users/al/Desktop /volumes/owc-1/backup-desktopstuff
#
touch /volumes/owc-1/backup-desktopstuff/last-update.txt /volumes/owc-1/backup-desktopstuff
#
# Back up ClientPDF folder to OWC
#
#echo "=========================" >> /Users/al/documents/wget/rsync-clientpdf-log.txt
#echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-clientpdf-log.txt
#echo "=========================" >> /Users/al/documents/wget/rsync-clientpdf-log.txt
#
# rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-clientpdf-log.txt /Users/al/ClientPDF /volumes/owc-1/backup-clientpdf 
#
#touch /volumes/owc-1/backup-clientpdf/last-update.txt /volumes/owc-1/backup-clientpdf
#
# Back up Music to OWC
#
echo "=====LOCAL=Music============" >> /Users/al/documents/wget/rsync-music-log.txt
echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-music-log.txt
echo "=========================" >> /Users/al/documents/wget/rsync-music-log.txt
#
rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-music-log.txt /Users/al/Music /volumes/owc-1/backup-music 
#
touch /volumes/owc-1/backup-music/last-update.txt /volumes/owc-1/backup-music
#
# Back up Pictures to OWC
#
echo "=======LOCAL=Pictures=========" >> /Users/al/documents/wget/rsync-pictures-log.txt
echo "======= $(date) ====" >>/Users/al/documents/wget/rsync-pictures-log.txt
echo "=========================" >> /Users/al/documents/wget/rsync-pictures-log.txt
#
rsync -aPq  --delete --log-file=/Users/al/documents/wget/rsync-pictures-log.txt /Users/al/Pictures /volumes/owc-1/backup-pictures 
#
touch /volumes/owc-1/backup-pictures/last-update.txt /volumes/owc-1/backup-pictures
#

#
else
echo "======No Device Found========" >> /Users/al/documents/wget/rsync-address-log.txt
echo "======No Device Found========" >> /Users/al/documents/wget/rsync-documents-log.txt
echo "======No Device Found========" >> /Users/al/documents/wget/rsync-desktopstuff-log.txt
# echo "======No Device Found========" >> /Users/al/documents/wget/rsync-current-desktop-log.txt
echo "======No Device Found========" >> /Users/al/documents/wget/rsync-music-log.txt
echo "======No Device Found========" >> /Users/al/documents/wget/rsync-pictures-log.txt
fi

#echo "----done----and ---- done"
