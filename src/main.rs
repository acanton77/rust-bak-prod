// RS_BAK_PROD -- RUST 
// RS_BAK_PROD -- RUST
// RS_BAK_PROD -- RUST

// Change Date: Tue Mar 12 08:14:26 PDT 2024
// Change title of email

// format code in BBEdit - find: //at-sign
//replace: //at-sign\n

#![allow(warnings)]

use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::US::Pacific;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use std::{thread, time};

use std::fs::rename;
use std::process::Stdio;

const SSH_RSYNC_ADDRESS: &str = "fm1364@fm1364.rsync.net";
const RSYNC_ADDRESS: &str = "fm1364@fm1364.rsync.net:";
const SWITCH_FILE: &str = "/usr/home/ancnet1/rs_bak_prod/bak_files/switch_file.txt";
const ZIP_IN_DIR: &str = "/usr/home/ancnet1/public_html/";
const HOME_FILE_DIR: &str = "/usr/home/ancnet1/rs_bak_prod/bak_files/";
const LINODE_DIR: &str =
    "/usr/home/ancnet1/rs_bak_prod/bak_files/Linode-Docs/Pair-cron/pair-anc1-cron.txt";

fn main() {
    let mut msg_vec: Vec<(String, String, String)> = Vec::new();
    let mut message_data = String::from("");
    let mut html_dir_name = String::from("");
    let mut zip_in_file = String::from("");
    let mut zip_out_file_name = String::from("");
    let mut rsync_dir = String::from("");
    let mut database = String::from("");
    let mut log_path = String::from("");
    let mut dump_out_file_name = String::from("");

    // Put one-time run here
    // =========

    // return;

    //==========

    // CREATE THE VEC FROM THE SWITCH FILE (call function)

    let mut vec_switch_file = lines_from_file(SWITCH_FILE).expect("Could not load lines");

    //*** APPEND DAILY ITEMS TO VEC_SWITCH_FILE

    vec_switch_file.push("linode".to_string());
    vec_switch_file.push("ac_addressbook".to_string());
    vec_switch_file.push("chk_espo_ver".to_string());
    vec_switch_file.push("jane".to_string());
    vec_switch_file.push("baikal".to_string());

    //*** SORT THE VECTOR - items are done in alpha order and so 1test will sort
    //    to the top if included in the switch file.

    vec_switch_file.sort();

    //*** DE-DUP THE VECTOR - We could get dupe espo or other jobs in the switch
    //    file so we need to eliminate those.

    vec_switch_file.dedup();

    //====write message for email ========================

    message_data = "vec_switch and appends DONE".to_string();
    write_msg(&mut msg_vec, message_data);
    //@



    //***********************LOOP *********************************
    //***********************LOOP *********************************
    //***********************LOOP *********************************

    //*** On all files and directories that we upload we append a timestamp.
    //@



    //=== 1TEST
    //=== 1TEST
    //=== 1TEST

    // Just send email if this is found in switch file

    for line in &vec_switch_file {
        if line == "1test" {
            message_data = "test1 - mail only".to_string();
            write_msg(&mut msg_vec, message_data);
            break;
        } // end 1test
          //@



        //=== ADAMS BLAKE
        //=== ADAMS BLAKE
        //=== ADAMS BLAKE

        if line == "adams" {
            zip_in_file = "/usr/home/ancnet1/public_html/adams-blake.com".to_string();
            zip_out_file_name = "adams-blake.com-rs-".to_string();
            message_data = "adams-blake backup is DONE".to_string();
            rsync_dir = "web-backup-adams-blake".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end adams
          //@



        //=== ALICEGIFT
        //=== ALICEGIFT
        //=== ALICEGIFT

        // this is a wordpress site
        if line == "alicegift" {
            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/shop".to_string();
            zip_out_file_name = "alicegift.com-rs-".to_string();
            message_data = "alicegift site backup is DONE".to_string();
            rsync_dir = "web-backup-alicegift".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);

            database = "ancnet1_shop".to_string();
            dump_out_file_name = "alicegift-rs-db-".to_string();
            log_path = "--login-path=shop".to_string();
            bak_wordpress(database, dump_out_file_name, log_path, &rsync_dir);
            message_data = "alicegift db backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // end alicegift
          //@



        //=== ANCINS
        //=== ANCINS
        //=== ANCINS

        if line == "ancins" {
            zip_in_file = "/usr/home/ancnet1/public_html/ancins.com".to_string();
            zip_out_file_name = "ancins.com-rs-".to_string();
            message_data = "ancins backup is DONE".to_string();
            rsync_dir = "web-backup-ancins".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end ancins
          //@



        //=== ANGELPUP
        //=== ANGELPUP
        //=== ANGELPUP

        if line == "angelpup" {
            zip_in_file = "/usr/home/ancnet1/public_html/angelpup.com".to_string();
            zip_out_file_name = "angelpup.com-rs-".to_string();
            message_data = "angelpup backup is DONE".to_string();
            rsync_dir = "web-backup-angelpup".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end angelpup
          //@



        //=== ANSWER123
        //=== ANSWER123
        //=== ANSWER123

        if line == "answer123" {
            zip_in_file = "/usr/home/ancnet1/public_html/answer123.com".to_string();
            zip_out_file_name = "answer123.com-rs-".to_string();
            message_data = "answer123 backup is DONE".to_string();
            rsync_dir = "web-backup-answer123".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end angelpup
          //@



        //=== ARTFROMAMY
        //=== ARTFROMAMY
        //=== ARTFROMAMY

        // this is a wordpress site
        if line == "artfromamy" {
            zip_in_file = "/usr/home/ancnet1/public_html/artfromamy.com".to_string();
            zip_out_file_name = "artfromamy.com-rs-".to_string();
            message_data = "artfromamy site backup is DONE".to_string();
            rsync_dir = "web-backup-artfromamy".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);

            database = "ancnet1_artforamy2".to_string();
            dump_out_file_name = "artfromamy-rs-db-".to_string();
            log_path = "--login-path=artfromamy".to_string();
            bak_wordpress(database, dump_out_file_name, log_path, &rsync_dir);
            message_data = "artfromamy db backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // end artfromamy
          //@



        //=== ESPO DB
        //=== ESPO DB
        //=== ESPO DB

        // Since we have to dump the database and then update the backup database
        // we use a bash script which is more 'trustworthy' than Rusts 'process/cmd.'

        if line == "espodb" {
            let timestamp_espo = get_timestamp().to_string();
            let mut commandx = Command::new("bash");
            commandx.args([
                "/usr/home/ancnet1/rs_bak_prod/bak_files/bash/espodb.sh",
                &timestamp_espo,
            ]);
            commandx.output().expect("Failed to execute espodb command");

            let mut espo_vec: Vec<String> = Vec::new();

            for file in
                fs::read_dir("/usr/home/ancnet1/rs_bak_prod/bak_files/espo-db-backup-dir").unwrap()
            {
                espo_vec.push(file.unwrap().path().display().to_string())
            }

            &espo_vec.sort();

            &espo_vec.sort();

            &espo_vec.reverse();
            let count2 = espo_vec.len();

            for i in 4..count2 {
                // println!("deleted: {}", &espo_vec[i]);
                let mut file_name_to_be_deleted2 = &espo_vec[i];
                fs::remove_file(file_name_to_be_deleted2);
                // println!("file-to-be-deleted: {}", file_name_to_be_deleted2);
            }

            // Send the whole directory up to the datacenter
            let _cmd = Command::new("rsync")
                .args([
                    "-a",
                    "-r",
                    "-q",
                    "--delete",
                    "/usr/home/ancnet1/rs_bak_prod/bak_files/espo-db-backup-dir",
                    "fm1364@fm1364.rsync.net:espo-db-backup-rs",
                ])
                .output()
                .expect("rsync command failed to start");

            message_data = "Espo DB backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // end espodb
          //@



        //=== ESPO SITE
        //=== ESPO SITE
        //=== ESPO SITE

        if line == "esposite" {
            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/espocrm2".to_string();
            zip_out_file_name = "esposite2-rs-".to_string();
            message_data = "espo site backup is DONE".to_string();
            rsync_dir = "web-backup-esposite".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end espo site
          //@



        //=== K6AAI
        //=== K6AAI
        //=== K6AAI
        //@



        if line == "k6aai" {
            zip_in_file = "/usr/home/ancnet1/public_html/k6aai.net".to_string();
            zip_out_file_name = "k6aai.net-rs-".to_string();
            message_data = "k6aai backup is DONE".to_string();
            rsync_dir = "web-backup-k6aai".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end K6AAI
          //@



        //=== NEWMEDIAECOM
        //=== NEWMEDIAECOM
        //=== NEWMEDIAECOM

        if line == "newmediaecom" {
            zip_in_file = "/usr/home/ancnet1/public_html/newmediaecom.com".to_string();
            zip_out_file_name = "newmediaecom.com-rs-".to_string();
            message_data = "newmediaecom backup is DONE".to_string();
            rsync_dir = "web-backup-newmediaecom".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end newmediaecom
          //@



        //=== NEWMEDIALANDING
        //=== NEWMEDIALANDING
        //=== NEWMEDIALANDING

        if line == "newmedialanding" {
            zip_in_file = "/usr/home/ancnet1/public_html/newmediacreate.com".to_string();
            zip_out_file_name = "newmediacreate.com-rs-".to_string();
            message_data = "newmediacreate backup is DONE".to_string();
            rsync_dir = "web-backup-newmediacreate".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end newmedialanding
          //@



        //=== NEWMEDIALITE
        //=== NEWMEDIALITE
        //=== NEWMEDIALITE

        if line == "newmedialite" {
            zip_in_file = "/usr/home/ancnet1/public_html/newmedialite.com".to_string();
            zip_out_file_name = "newmedialite.com-rs-".to_string();
            message_data = "newmedialite backup is DONE".to_string();
            rsync_dir = "web-backup-newmedialite".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end newmedialanding
          //@



        //=== NEWMEDIAWEBSITEDESIGN
        //=== NEWMEDIAWEBSITEDESIGN
        //=== NEWMEDIAWEBSITEDESIGN

        if line == "newmediawebsitedesign" {
            zip_in_file = "/usr/home/ancnet1/public_html/newmediawebsitedesign.com".to_string();
            zip_out_file_name = "newmediawebsitedesign.com-rs-".to_string();
            message_data = "newmediawebsitedesign backup is DONE".to_string();
            rsync_dir = "web-backup-newmediawebsitedesign".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end newmedialanding
          //@



        //=== PIWIGOSITE
        //=== PIWIGOSITE
        //=== PIWIGOSITE

        if line == "pi1wigo-site" {
            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/piwigopix".to_string();
            zip_out_file_name = "piwigosite-rs-".to_string();
            message_data = "piwigosite backup is DONE".to_string();
            rsync_dir = "web-backup-piwigo".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end piwigo
          //@



        if line == "piwigo-db" {
            database = "ancnet1_piwigotest".to_string();
            dump_out_file_name = "piwigo-db-rs-".to_string();
            rsync_dir = "web-backup-piwigo".to_string();
            log_path = "--login-path=piwigo".to_string();
            bak_wordpress(database, dump_out_file_name, log_path, &rsync_dir);
            message_data = "piwigo-db backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // end piwigo-db
          //@



        //=== RADIOQSL
        //=== RADIOQSL
        //=== RADIOQSL

        if line == "radioqsl" {
            zip_in_file = "/usr/home/ancnet1/public_html/radioqsl.com".to_string();
            zip_out_file_name = "radioqsl.com-rs-".to_string();
            message_data = "radioqsl backup is DONE".to_string();
            rsync_dir = "web-backup-radioqsl".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end radioqsl
          //@



        //=== SCRIPTS-RS
        //=== SCRIPTS-RS
        //=== SCRIPTS-RS

        if line == "scripts" {
            zip_in_file = "/usr/home/ancnet1/rs_bak_prod/src".to_string();
            zip_out_file_name = "rs_bak_prod_src-".to_string();
            message_data = "rs_bak_prod_src backup is DONE".to_string();
            rsync_dir = "web-backup-scripts".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end scripts-rs
          //@



        //=== WRITINGS
        //=== WRITINGS
        //=== WRITINGS

        if line == "writings" {
            zip_in_file = "/usr/home/ancnet1/public_html/acwritings.com".to_string();
            zip_out_file_name = "writings.com-rs-".to_string();
            message_data = "writings backup is DONE".to_string();
            rsync_dir = "web-backup-writings".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end writings
          //@



        //=== YOURLS

        if line == "yourls" {
            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/urls9296".to_string();
            zip_out_file_name = "yourls-rs-".to_string();
            message_data = "yourls backup is DONE".to_string();
            rsync_dir = "web-backup-yourls".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end yourls
          //@



        //=== LINODE
        //=== LINODE
        //=== LINODE

        // Do a crontab dump to file -- see global constant
        //*** Because we have to capture the output using a bash script is less complex

        if line == "linode" {
            /*
                       let file_lin = File::create(LINODE_DIR).unwrap();
                       let stdio = Stdio::from(file_lin);

                       let _cmd3 = Command::new("/usr/bin/crontab")
                           .stdout(stdio)
                           .args(["-l"])
                           .output()
                           .expect("crontab command failed to start");
            */

            let mut commandx = Command::new("bash");
            commandx.args([
                "/usr/home/ancnet1/rs_bak_prod/bak_files/bash/linode-cron.sh",
                LINODE_DIR,
            ]);
            commandx.output().expect("Failed to execute command");

            // zip the Linode docs dir

            zip_in_file = "/usr/home/ancnet1/rs_bak_prod/bak_files/Linode-Docs".to_string();
            zip_out_file_name = "linode-docs-rs-".to_string();
            rsync_dir = "web-backup-linode-rs".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            message_data = "linode backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // end linode
          //@



        //=== CHECK ESPO
        //=== CHECK ESPO
        //=== CHECK ESPO

        //*** Special logic to scrape web page to find latest version and compare
        //    to what we had. We keep previous version in a .txt file

        // Use curl to get the web page (probably should have used bash)

        if line == "chk_espo_ver" {
            let _mycurl = Command::new("curl")
                .args([
                    "https://www.espocrm.com/download/",
                    "-o",
                    "/usr/home/ancnet1/rs_bak_prod/bak_files/grep1.txt",
                ])
                .output()
                .expect("curl command failed to start");

            // Use grep to find the string

            let cmdx = Command::new("/usr/bin/grep")
                .args([
                    "-i",
                    "Latest Release EspoCRM",
                    "/usr/home/ancnet1/rs_bak_prod/bak_files/grep1.txt",
                ])
                .output()
                .expect("grep command failed to start");

            // Left in println! for debugging if needed.

            //println!("status: {}", cmdx.status);
            // println!("stdout: {}", String::from_utf8_lossy(&cmdx.stdout));
            // println!("err: {}", String::from_utf8_lossy(&cmdx.stderr));

            let new_ver = String::from_utf8_lossy(&cmdx.stdout);

            //===println!("{}", new_ver);
            let new_ver = new_ver.trim();
            //====println!("{}", new_ver);
            //println!("new_ver is: {}", new_ver);
            // prints out:  <h2>Latest Release EspoCRM 8.1.4 (February 07, 2024)</h2>

            // Get the previous version from the saved .txt file

            let mut prev_ver1 = String::from("");
            prev_ver1 =
                fs::read_to_string("/usr/home/ancnet1/rs_bak_prod/bak_files/prev-espo-ver.txt")
                    .expect("Should have been able to read the file");

            // get rid of \n as grep does not have one.

            let mut prev_ver = String::from("");
            prev_ver = prev_ver1.to_string();
            //====println!("---prev--");
            //====println!("{}", prev_ver);
            prev_ver = prev_ver.trim().to_string();
            //prev_ver.pop();
            //====println!("{}", prev_ver);
            // If no new version, all done, else
            if new_ver == prev_ver {
                //println!("they are equal");
                message_data = "espo: new version NOT FOUND ".to_string();
                write_msg(&mut msg_vec, message_data);

            //*** if new version, write the message. We don't do update. Done in
            //    another script the actually updates the Espo system.
            } else {
                message_data = "espo: new version FOUND. -- Do the manual update. ".to_string();
                write_msg(&mut msg_vec, message_data);

                message_data = format!("version is: {}", new_ver.to_string());
                write_msg(&mut msg_vec, message_data);
            } // end if-else
        } // end chk-espo
          //@



        //=== ADDRESS BOOKS==========
        //=== ADDRESS BOOKS==========
        //=== ADDRESS BOOKS==========

        if line == "ac_addressbook" {
            // *** We upload the previously made vcf file. We get the directory with the
            //     BusyContact files and we only keep the latest 4 of them.

            let mut my_vec: Vec<String> = Vec::new();

            //*** The BusyContact address book has dir names with spaces.
            //    We open the dir with them and read them into some container
            //    called 'files'. We then replace those with spaces with - (dash)

            for file in fs::read_dir(
                "/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs/BusyContacts",
            )
            .unwrap()
            {
                let mut orig_file_name: String = file.unwrap().path().display().to_string();
                let mut new_file_name: String = "".to_string();

                if orig_file_name.contains("babu") {
                    if orig_file_name.contains(" ") {
                        new_file_name = orig_file_name.replace(" ", "-");

                        let _ = rename(orig_file_name, new_file_name.clone());

                        my_vec.push(new_file_name.to_string());
                    } else {
                        my_vec.push(orig_file_name.to_string());
                    }
                } // outer if
            } // for loop

            // we want to get rid of all that are too old.

            // Sort the vec of file names into descending: old first

            &my_vec.sort();

            // Now reorder them so that the newest are at the top

            &my_vec.reverse();

            // We are going to find out how many we have in the vec and then only keep
            // the top (newest) for files by deleting all the others.

            let count = my_vec.len();

            for i in 4..count {
                let mut file_name_to_be_deleted = &my_vec[i];
                fs::remove_dir_all(file_name_to_be_deleted);
            }

            //*** write our mail message, touch a file to show date done and send up to datacenter

            message_data = "address book backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);

            //*** We touch a file as a time indicator/documentation

            let _cmd = Command::new("touch")
                .args(["/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs/rs-address-time.txt",
                ])
                .output()
                .expect("touch command failed to start");

            //*** Send the whole directory up to the datacenter

            let _cmd = Command::new("rsync")
                .args([
                    "-a",
                    "-r",
                    "-q",
                    "--delete",
                    "/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs",
                    "fm1364@fm1364.rsync.net:addressbook-backup-rs",
                ])
                .output()
                .expect("rsync command failed to start");

            // touch the datacenter address book folder for new date

            let _cmd = Command::new("/usr/bin/ssh")
                .args([
                    SSH_RSYNC_ADDRESS.to_string(),
                    " touch ".to_string(),
                    "addressbook-backup-rs".to_string(),
                ])
                .output()
                .expect("ssh command failed to start");
        } // address book
          //@



        // ======JANE ADDRESS BOOK
        // ======JANE ADDRESS BOOK
        // ======JANE ADDRESS BOOK

        if line == "jane" {
            let _cmd = Command::new("rsync")
                .args([
                    "-a",
                    "-r",
                    "-q",
                    "--delete",
                    "/usr/home/ancnet1/py-backup/bak-files/address-book-backup-dir/JaneAddressFolder",
                    "fm1364@fm1364.rsync.net:addressbook-backup-rs",
                ])
                .output()
                .expect("rsync command failed to start");

            message_data = "Jane address backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        } // jane
          //@



        // ======BAIKAL ADDRESS BOOK
        // ======BAIKAL ADDRESS BOOK
        // ======BAIKAL ADDRESS BOOK

        //usr/home/ancnet1/public_html/anc123.com/baikal94a

        if line == "baikal" {
            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/baikal94a".to_string();
            zip_out_file_name = "baikal94a-rs-".to_string();
            message_data = "baikal backup is DONE".to_string();
            rsync_dir = "web-backup-baikal-rs".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end baikal
          //@



        //=== ANC123
        //=== ANC123
        //=== ANC123

        if line == "anc123" {
            let timestamp_anc123 = get_timestamp().to_string();
            let mut anc123_zip_file = "/usr/home/ancnet1/rs_bak_prod/anc123.com-rs-".to_string();
            anc123_zip_file.push_str(&timestamp_anc123);
            anc123_zip_file.push_str(".zip");

            let _cmd2 = Command::new("/usr/bin/zip")
                .args([
                    "-rq",
                    &anc123_zip_file,
                    "/usr/home/ancnet1/public_html/anc123.com",
                    "-x",
                    "/usr/home/ancnet1/public_html/anc123.com/baikal94a/*",
                    "/usr/home/ancnet1/public_html/anc123.com/espocrm2/*",
                    "/usr/home/ancnet1/public_html/anc123.com/piwigopix/*",
                    "/usr/home/ancnet1/public_html/anc123.com/shop/*",
                    "/usr/home/ancnet1/public_html/anc123.com/urls9296/*",
                ])
                .output()
                .expect("zip command failed to start");

            /*  NOT USED.
                       let mut commandx = Command::new("bash");
                       commandx.args([
                           "/usr/home/ancnet1/rs_bak_prod/bak_files/bash/anc123.sh",
                           &anc123_zip_file,
                       ]);
                       commandx.output().expect("Failed to execute espodb command");
            */

            //*** delete older files on datacenter via SSH rm command

            let _cmd = Command::new("/usr/bin/ssh")
                .args([
                    SSH_RSYNC_ADDRESS.to_string(),
                    " rm ".to_string(),
                    "web-backup-anc123/*.zip".to_string(),
                ])
                .output()
                .expect("ssh command failed to start");

            // Send the zip up to the datacenter
            let _cmd = Command::new("rsync")
                .args([
                    "-a",
                    "-r",
                    "-q",
                    "--delete",
                    &anc123_zip_file,
                    "fm1364@fm1364.rsync.net:web-backup-anc123",
                ])
                .output()
                .expect("rsync command failed to start");

            message_data = "anc123 backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);

            //*** Delete the local .zip file

            fs::remove_file(anc123_zip_file);
        } // end anc123
          //@



        //=== NEXT SITE==========

        //if line == "put next site here" {

        //} // end next site
    } // end for LOOP

    //*********************** END LOOP *********************************
    //*********************** END LOOP *********************************
    //*********************** END LOOP *********************************
    //*** All done with processing so we call the send-mail function

    send_mail(&mut msg_vec, &vec_switch_file);

    //*** truncate switch file as we clean it out for next time.

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(SWITCH_FILE);

    // Write message to a the crontab file

    let current_local_done: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    let msg_date_done = current_local_done.format("%c");

    println!("---GOOD EOJ---");
    println!("Date: {}", msg_date_done);
}

// ****************** FUNCTIONS *************************************************
// ****************** FUNCTIONS *************************************************
// ****************** FUNCTIONS *************************************************
//@



//=== FUNCTION: BOOTSTRAP ================================================
//=== FUNCTION: BOOTSTRAP ================================================

fn bak_bootstrap(mut zip_in_file: String, mut zip_out_file_name: String, mut rsync_dir: &String) {
    //** create the output file name that is zipped.

    let mut zip_out_file: String = format!("{}{}", HOME_FILE_DIR.to_string(), zip_out_file_name);
    let timestamp_new = get_timestamp().to_string();

    zip_out_file.push_str(&timestamp_new);
    zip_out_file.push_str(".zip");

    // zip up the file (zip file name comes first, file-to-zip comes second

    let _cmd2 = Command::new("/usr/bin/zip")
        .args(["-r", &zip_out_file, &zip_in_file])
        .output()
        .expect("zip command failed to start");

    // Remove previous  zip files on rsync datacenter

    //*** "ssh fm1364@fm1364.rsync.net 'rm  web-backup-baikal/baikal*.zip'"

    let del_rsync_file: String = format!(
        "{}{}{}{}",
        &rsync_dir.to_string(),
        "/",
        &zip_out_file_name.to_string(),
        "*".to_string()
    );

    let _cmd = Command::new("/usr/bin/ssh")
        .args([
            SSH_RSYNC_ADDRESS.to_string(),
            " rm ".to_string(),
            del_rsync_file,
        ])
        .output()
        .expect("ssh command failed to start");

    // Left in for debugging if necessary
    //println!(
    // "stderr from  function: {}",
    //  String::from_utf8_lossy(&_cmd.stderr)
    //  );

    //  println!("stdout: {}", String::from_utf8_lossy(&_cmd.stdout));
    //   println!("status: {}", _cmd.status);
    //  println!("{:?}", _cmd);

    //*** Now we send the zip file up to the rsync.net datacenter

    let rsync_out: String = RSYNC_ADDRESS.to_string() + &rsync_dir.to_string();

    let _cmd = Command::new("rsync")
        .args(["-r", "-a", "-p", &zip_out_file, &rsync_out])
        .output()
        .expect("rsync command failed to start");

    //*** Delete the local .zip file

    fs::remove_file(zip_out_file);
} //end bootstrap
  //@



//=== FUNCTION: WORDPRESS ===========================
//=== FUNCTION: WORDPRESS ===========================

fn bak_wordpress(
    database: String,
    dump_out_file_name: String,
    log_path: String,
    rsync_dir: &String,
) {
    //*** set up the timestamp

    let timestamp_new = get_timestamp().to_string();
    let current_local22: DateTime<Local> = Local::now();
    let custom_format22 = current_local22.format("%b %d %H:%M:%S");

    //*** set up the file name with timestamp and sql

    let mut dump_out_file: String = format!("{}{}", HOME_FILE_DIR.to_string(), dump_out_file_name);
    dump_out_file.push_str(&timestamp_new);
    dump_out_file.push_str(".sql");

    let mut dump_out_file2 = "--result-file=".to_string();
    dump_out_file2.push_str(&dump_out_file);

    //*** dump database to the sql file -

    let _cmd = Command::new("/usr/local/bin/mysqldump")
        .args([
            log_path,
            "--add-drop-table".to_string(),
            database,
            dump_out_file2,
        ])
        .output()
        .expect("mysql command failed to start");

    //  zip up the database file - zip file comes first, then the input file

    let mut db_zip: String = dump_out_file.to_string();
    let mut sql_in: String = dump_out_file.to_string();

    db_zip.push_str(".zip");

    let _cmd2 = Command::new("/usr/bin/zip")
        .args(["-r", &db_zip, &sql_in])
        .output()
        .expect("zip command failed to start");

    //*** Use SSH to remove previous  zip files on rsync datacenter

    let del_rsync_file: String = format!(
        "{}{}{}{}",
        &rsync_dir.to_string(),
        "/",
        &dump_out_file_name.to_string(),
        "*".to_string()
    );

    let _cmd = Command::new("/usr/bin/ssh")
        .args([
            SSH_RSYNC_ADDRESS.to_string(),
            " rm ".to_string(),
            del_rsync_file,
        ])
        .output()
        .expect("ssh command failed to start");

    //*** for debugging if needed
    //println!(
    // "stderr from  function: {}",
    //  String::from_utf8_lossy(&_cmd.stderr)
    //  );
    //  println!("stdout: {}", String::from_utf8_lossy(&_cmd.stdout));
    //   println!("status: {}", _cmd.status);

    //  println!("{:?}", _cmd);

    // rsync the zip db zip file

    //*** upload the sql zip file to datacenter

    let rsync_out = RSYNC_ADDRESS.to_string() + &rsync_dir.to_string();

    let _cmd = Command::new("rsync")
        .args(["-r", "-a", "-p", &db_zip, &rsync_out])
        .output()
        .expect("rsync command failed to start");

    // Delete the .zip file

    fs::remove_file(db_zip);
    fs::remove_file(sql_in);
} // end wordpress
  //@



//=== FUNCTION: CURRENT DATE ================
//=== FUNCTION: CURRENT DATE ================

fn get_current_date() -> String {
    let current_local: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    let msg_date1 = current_local.format("%Y-%m-%d %H:%M:%S:%3f %Z");
    msg_date1.to_string()
} // end current date
  //@



//=== FUNCTION: TIME STAMP =======================
//=== FUNCTION: TIME STAMP =======================
//@



fn get_timestamp() -> String {
    let milliseconds_timestamp: u128 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let timestamp_string: String;
    timestamp_string = milliseconds_timestamp.to_string();

    timestamp_string // return
} // end time stamp
  //@



//=== FUNCTION: CREATE VEC FROM SWITCH FILE=========
//=== FUNCTION: CREATE VEC FROM SWITCH FILE=========

//*** This reads the switch file into a vector used to find out what sties
//    to update via the big loop

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
} // end create vec
  //@



// FUNCTION: WRITE MESSAGE TO MSG_VEC
// FUNCTION: WRITE MESSAGE TO MSG_VEC
// FUNCTION: WRITE MESSAGE TO MSG_VEC

//*** Send the message string to vector that holds these for later reporting/email.
//    The vector is a tupple formatted as timestamp, regular date, and message.
//    Timestamp is used to sort these later on. It is removed.

fn write_msg(msg_vec: &mut Vec<(String, String, String)>, message_data: String) {
    thread::sleep(Duration::from_millis(100));
    let msg_date: String = get_current_date();
    let time_stamp = get_timestamp();
    msg_vec.push((time_stamp.to_string(), msg_date.to_string(), message_data));
}

// FUNCTION: SEND MAIL
// FUNCTION: SEND MAIL
// FUNCTION: SEND MAIL

//*** Read the message vector and formats a string with all the content lines
//    for the email to go out.

fn send_mail(msg_vec: &mut Vec<(String, String, String)>, vec_switch_file: &Vec<(String)>) {
    let mut msg_final = String::new();

    //let msg_date_mail: String = get_current_date();

    let current_local: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    let msg_date_mail = current_local.format("%c");

    let msg_line_date = format!("Sent on {} by Rust script on Pair VPS.\n", msg_date_mail);

    msg_final.push_str(&msg_line_date);

    msg_final.push_str("Jobs run in Mountain time (+1 hr.) but reported in Pacific time.\n");

    msg_final.push_str("\n");

    msg_final.push_str("Files to be done:\n");

    msg_final.push_str("\n");

    //*** this is a loop to read the switch vector file to see what sites were backed  up

    for line in vec_switch_file {
        let newvalue2 = format!("{}\n", line);
        msg_final.push_str(&newvalue2);
    } // end for line read loop

    msg_final.push_str("\n");

    // get the msg file and pop in line numbrers

    let mut num = 0;
    let mut newvalue: String;
    let mut from_orig: String;

    //*** We read the messages for each site done and add line numbers. We drop the
    //    datestamp as can be seen in the format code.

    for line in msg_vec {
        num = num + 1;
        let mut num2: &str = &num.to_string();
        let newvalue = format!("   {}--  {}  {}\n", num2, line.1, line.2);
        msg_final.push_str(&newvalue);
    } // end for msg_vec line read loop

    msg_final.push_str("\n");

    //msg_final.push_str("Errors:\n");
    msg_final.push_str("\n");

    //No errors found.

    msg_final.push_str("Note:\n\n");

    msg_final.push_str("# For user crontab - pair:\n");

    msg_final.push_str("crontab -e \n");
    msg_final.push_str("crontab -l \n\n");

    msg_final.push_str("# For root crontab - not used:\n");
    msg_final.push_str("sudo nano /etc/crontab\n\n");

    msg_final.push_str("# To update ESPO to a new version:\n");
    msg_final.push_str("SSH into Pair VPS\n");
    msg_final.push_str("cd /usr/home/ancnet1/rs_bak_prod/bak_files/bash\n");
    msg_final.push_str("./espo-prod-new-ver.sh \n\n");

    msg_final.push_str("# To backup any site or file:\n");
    msg_final.push_str("Go to: https://anc123.com/switch3/index.php\n");
    msg_final.push_str("Enter the job/backups you want to run\n");
    msg_final.push_str("The backup will be uploaded to rsync.net that night.\n\n");

    msg_final.push_str("-- End --\n");
    //@



    //*** Email code

    let email = Message::builder()
        .from("Pair-Rust-VPS <ac99@answer123.com>".parse().unwrap())
        .to("ANC <ac99@answer123.com>".parse().unwrap())
        .subject("Backups --  Rust")
        .header(ContentType::TEXT_PLAIN) // MUST HAVE THIS OR LINES TRUNCATE
        .body(String::from(msg_final))  // GET FROM STRING ABOVE
        .unwrap();

    let creds = Credentials::new(
        "smtp2@mailanc2.net".to_string(),
        "tek7iah77mail-".to_string(),
    );

    // Open a remote connection to Pair server

    let mailer = SmtpTransport::relay("ancnet1.mail.pairserver.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email and write a message which will be in a file

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        // Ok(_) => a = 2,
        Err(e) => panic!("Could not send email: {:?}", e),
    } // end match
} // end mail
  //} // end all code
