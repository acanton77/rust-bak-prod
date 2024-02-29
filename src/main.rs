// RS_BAK_PROD -- Rust
// RS_BAK_PROD -- RUST
// RS_BAK_PROD -- RUST

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

    //==========

    // CREATE THE VEC FROM THE SWITCH FILE (call function)

    let mut vec_switch_file = lines_from_file(SWITCH_FILE).expect("Could not load lines");

    //*** APPEND DAILY ITEMS TO VEC_SWITCH_FILE

    vec_switch_file.push("linode".to_string());
    vec_switch_file.push("ac_addressbook".to_string());
    vec_switch_file.push("chk_espo_ver".to_string());
    //vec_switch_file.push("1test".to_string());

    //*** SORT THE VECTOR

    vec_switch_file.sort();

    //*** DE-DUP THE VECTOR

    vec_switch_file.dedup();

    //====write message for email ========================

    message_data = "vec_switch and appends DONE".to_string();
    write_msg(&mut msg_vec, message_data);

    //***********************LOOP *********************************
    //***********************LOOP *********************************
    //***********************LOOP *********************************

    //=== 1TEST==========
    // Just send email if this is found in switch file
    for line in &vec_switch_file {
        if line == "1test" {
            message_data = "test1 - mail only".to_string();
            write_msg(&mut msg_vec, message_data);
            break;
        }
        //=== LINODE==========

        // Do a crontab dump to file -- see global constant
        if line == "linode" {
            let file_lin = File::create(LINODE_DIR).unwrap();
            let stdio = Stdio::from(file_lin);

            let _cmd3 = Command::new("/usr/bin/crontab")
                .stdout(stdio)
                .args(["-l"])
                .output()
                .expect("crontab command failed to start");

            zip_in_file = "/usr/home/ancnet1/rs_bak_prod/bak_files/Linode-Docs".to_string();
            zip_out_file_name = "linode-docs-rs-".to_string();
            rsync_dir = "web-backup-linode-rs".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            message_data = "linode backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);
        }

        //=== PIWIGO==========

        if line == "piwigo" {
            // println!("we got piwigo");
        }

        // this is a wordpress site.
        if line == "alicegift" {
            // println!("we will do alicegift");
            // alicegift(&mut msg1);
        }

        //=== ANCINS==========
        if line == "ancins" {
            zip_in_file = "/usr/home/ancnet1/public_html/ancins.com".to_string();
            zip_out_file_name = "ancins.com-rs-".to_string();
            message_data = "ancins backup is DONE".to_string();
            rsync_dir = "web-backup-ancins".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        }

        //=== K6AAI==========

        if line == "k6aai" {
            zip_in_file = "/usr/home/ancnet1/public_html/k6aai.net".to_string();
            zip_out_file_name = "k6aai.net-rs-".to_string();
            message_data = "k6aai backup is DONE".to_string();
            rsync_dir = "web-backup-k6aai".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        }

        //=== ARTFROMAMY==========
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
        }

        //=== CHECK ESPO==========
        // Special logic to scrape web page to find latest version and compare to what we had.
        // Use curl to get the web page
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

            // for debugging if needed.
            //println!("status: {}", cmdx.status);
            // println!("stdout: {}", String::from_utf8_lossy(&cmdx.stdout));
            // println!("err: {}", String::from_utf8_lossy(&cmdx.stderr));

            let new_ver = String::from_utf8_lossy(&cmdx.stdout);

            //===println!("{}", new_ver);
            let new_ver = new_ver.trim();
            //====println!("{}", new_ver);
            //println!("new_ver is: {}", new_ver);
            // prints out:  <h2>Latest Release EspoCRM 8.1.4 (February 07, 2024)</h2>

            // Get the previous version from file
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
                message_data = "new espo version NOT FOUND ".to_string();
                write_msg(&mut msg_vec, message_data);

            // if new version, write the message. No longer do update
            } else {
                //println!("NOT EQUAL");

                /*
                               fs::write(
                                   "/usr/home/ancnet1/rs_bak_prod/bak_files/prev-espo-ver.txt",
                                   new_ver,
                               )
                               .expect("Unable to write file");
                */

                // println!("WROTE THE FILE");

                message_data = "new espo version FOUND. -- Do the manual update. ".to_string();
                write_msg(&mut msg_vec, message_data);

                message_data = format!("version is: {}", new_ver.to_string());
                write_msg(&mut msg_vec, message_data);
            } // end if-else
        } // end chk-espo

        //=== ADDRESS BOOKS==========

        if line == "ac_addressbook" {
            // send up the vcf file

            let mut my_vec: Vec<String> = Vec::new();
            //let mut z : String ="".to_string();
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

            // Sort the vec of file names into descending: old first

            //            println!("===========  descending vec ===========");
            &my_vec.sort();
            /*
                       for line in &my_vec {
                           println!("{}", line);
                       }
            */
            // Now reorder them so that the newest are at the top

            //           println!("===========  reverse vec ===========");
            &my_vec.reverse();
            /*
                       for line in &my_vec {
                           println!("{}", line);
                       }
            */

            //           println!("===========  only 1st 4 ===========");

            /*
                       for i in 0..4 {
                           // for line in &my_vec {
                           println!("{}", &my_vec[i]);

                       }
            */
            // We are going to find out how many we have in the vec and then only keep
            // the top (newest) for files by deleting all the others.
            //           println!("===========  From 4 to end ===========");
            let count = my_vec.len();

            // println!("busy conact dir count: {}", count);
            for i in 4..count {
                //  println!("deleted: {}", &my_vec[i]);
                let mut file_name_to_be_deleted = &my_vec[i];
                fs::remove_dir_all(file_name_to_be_deleted);
            }

            // write our mail message, touch a file to show date done and send up to datacenter
            message_data = "address book backup is DONE".to_string();
            write_msg(&mut msg_vec, message_data);

            let _cmd = Command::new("touch")
                .args(["/usr/home/ancnet1/rs_bak_prod/bak_files/address-book-backup-dir-rs/rs-address-time.txt",
                ])
                .output()
                .expect("touch command failed to start");

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

            // Jane address book

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

            // Baikal address book:  /usr/home/ancnet1/public_html/anc123.com/baikal94a

            zip_in_file = "/usr/home/ancnet1/public_html/anc123.com/baikal94a".to_string();
            zip_out_file_name = "baikal94a-rs-".to_string();
            message_data = "baikal backup is DONE".to_string();
            rsync_dir = "web-backup-baikal-rs".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        } // end addressabok

        //=== NEXT SITE==========

        if line == "put next site here" {} // end next site
    } // end for loop

    //*********************** END LOOP *********************************
    //*********************** END LOOP *********************************

    // Send the mail

    send_mail(&mut msg_vec, &vec_switch_file);

    //truncate switch file

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

//=== FUNCTION: BOOTSTRAP ================================================
//=== FUNCTION: BOOTSTRAP ================================================

fn bak_bootstrap(mut zip_in_file: String, mut zip_out_file_name: String, mut rsync_dir: &String) {
    let mut zip_out_file: String = format!("{}{}", HOME_FILE_DIR.to_string(), zip_out_file_name);
    let timestamp_new = get_timestamp().to_string();

    //====let current_local22: DateTime<Local> = Local::now();
    //let custom_format22 = current_local22.format("%Y-%m-%d %H:%M:%S");

    //=====let custom_format22 = current_local22.format("%b %d %H:%M:%S");
    //println!("current local 2: {}", custom_format22);

    zip_out_file.push_str(&timestamp_new);
    zip_out_file.push_str(".zip");

    // zip up the file (zip file comes first, file-to-zip comes second
    // zip "-r", "/usr/home/ancnet1/rs_bak/bak_files/out-dump.zip" "/usr/home/ancnet1/rs_bak/bak_files/out-dump.sql"

    let _cmd2 = Command::new("/usr/bin/zip")
        .args(["-r", &zip_out_file, &zip_in_file])
        .output()
        .expect("zip command failed to start");

    // Remove previous  zip files on rsync datacenter

    // python:  cmd = "ssh fm1364@fm1364.rsync.net 'rm  web-backup-baikal/baikal*.zip'"

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

    //println!(
    // "stderr from  function: {}",
    //  String::from_utf8_lossy(&_cmd.stderr)
    //  );

    //  println!("stdout: {}", String::from_utf8_lossy(&_cmd.stdout));
    //   println!("status: {}", _cmd.status);
    //  println!("{:?}", _cmd);

    // rsync

    let rsync_out: String = RSYNC_ADDRESS.to_string() + &rsync_dir.to_string();

    let _cmd = Command::new("rsync")
        .args(["-r", "-a", "-p", &zip_out_file, &rsync_out])
        .output()
        .expect("rsync command failed to start");

    // Delete the local .zip file

    fs::remove_file(zip_out_file);
} //end bootstrap

//=== FUNCTION: WORDPRESS ===========================
//=== FUNCTION: WORDPRESS ===========================

fn bak_wordpress(
    database: String,
    dump_out_file_name: String,
    log_path: String,
    rsync_dir: &String,
) {
    let timestamp_new = get_timestamp().to_string();
    let current_local22: DateTime<Local> = Local::now();
    let custom_format22 = current_local22.format("%b %d %H:%M:%S");

    // mysql dump

    let mut dump_out_file: String = format!("{}{}", HOME_FILE_DIR.to_string(), dump_out_file_name);

    dump_out_file.push_str(&timestamp_new);
    dump_out_file.push_str(".sql");

    let mut dump_out_file2 = "--result-file=".to_string();
    dump_out_file2.push_str(&dump_out_file);

    // zip the sql file

    let _cmd = Command::new("/usr/local/bin/mysqldump")
        .args([
            log_path,
            "--add-drop-table".to_string(),
            database,
            dump_out_file2,
        ])
        .output()
        .expect("mysql command failed to start");

    //  zip up the database file

    let mut db_zip: String = dump_out_file.to_string();
    let mut sql_in: String = dump_out_file.to_string();

    db_zip.push_str(".zip");

    let _cmd2 = Command::new("/usr/bin/zip")
        .args(["-r", &db_zip, &sql_in])
        .output()
        .expect("zip command failed to start");

    //# Use SSH to remove previous  zip files on rsync datacenter

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

    //println!(
    // "stderr from  function: {}",
    //  String::from_utf8_lossy(&_cmd.stderr)
    //  );
    //  println!("stdout: {}", String::from_utf8_lossy(&_cmd.stdout));
    //   println!("status: {}", _cmd.status);

    //  println!("{:?}", _cmd);

    // rsync the zip db zip file

    let rsync_out = RSYNC_ADDRESS.to_string() + &rsync_dir.to_string();

    let _cmd = Command::new("rsync")
        .args(["-r", "-a", "-p", &db_zip, &rsync_out])
        .output()
        .expect("rsync command failed to start");

    // Delete the .zip file

    fs::remove_file(db_zip);
    fs::remove_file(sql_in);
} // end wordpress

//=== FUNCTION: CURRENT DATE ================
//=== FUNCTION: CURRENT DATE ================

fn get_current_date() -> String {
    let current_local: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
    let msg_date1 = current_local.format("%Y-%m-%d %H:%M:%S:%3f %Z");
    msg_date1.to_string()
} // end current date

//=== FUNCTION: TIME STAMP =======================
//=== FUNCTION: TIME STAMP =======================

fn get_timestamp() -> String {
    let milliseconds_timestamp: u128 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let timestamp_string: String;
    timestamp_string = milliseconds_timestamp.to_string();

    timestamp_string // return
} // end time stamp

//=== FUNCTION: CREATE VEC FROM SWITCH FILE=========
//=== FUNCTION: CREATE VEC FROM SWITCH FILE=========

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
} // end create vec

// FUNCTION: WRITE MESSAGE TO MSG_VEC
// FUNCTION: WRITE MESSAGE TO MSG_VEC
// FUNCTION: WRITE MESSAGE TO MSG_VEC

fn write_msg(msg_vec: &mut Vec<(String, String, String)>, message_data: String) {
    thread::sleep(Duration::from_millis(100));
    let msg_date: String = get_current_date();
    let time_stamp = get_timestamp();
    msg_vec.push((time_stamp.to_string(), msg_date.to_string(), message_data));
}

// FUNCTION: SEND MAIL
// FUNCTION: SEND MAIL
// FUNCTION: SEND MAIL

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

    for line in vec_switch_file {
        let newvalue2 = format!("{}\n", line);
        msg_final.push_str(&newvalue2);
    } // end for line read loop

    msg_final.push_str("\n");

    // get the msg file and pop in line numbrers

    let mut num = 0;
    let mut newvalue: String;
    let mut from_orig: String;

    for line in msg_vec {
        num = num + 1;
        let mut num2: &str = &num.to_string();
        let newvalue = format!("   {}--  {}  {}\n", num2, line.1, line.2);

        //println!("line is: {}",newvalue);

        msg_final.push_str(&newvalue);
    } // end for line read loop

    msg_final.push_str("\n");

    msg_final.push_str("Errors:\n");
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
    msg_final.push_str("cd /usr/home/ancnet1/py-backup (or \'py\')\n");
    msg_final.push_str("./espo-prod-new-ver.sh \n\n");

    msg_final.push_str("# To backup any site or file:\n");
    msg_final.push_str("Go to: https://anc123.com/switch2/index.php\n");
    msg_final.push_str("Enter the job/backups you want to run\n");
    msg_final.push_str("The backup will be uploaded to rsync.net that night.\n\n");

    msg_final.push_str("-- End --\n");

    //msg_final.push_str("-------- END ----------\n");

    // println!("========== START MSG FILE ==========");

    // println!("{}", msg_final);
    // println!("========== END MSG FILE ==========");

    // Send the mail

    //println!("msg_final is: {}",msg_final);

    //*************************************************************
    //*****************EMAIL CODE**********************************
    //*************************************************************

    let email = Message::builder()
        .from("Pair-Rust-VPS <ac99@answer123.com>".parse().unwrap())
        .to("ANC <ac99@answer123.com>".parse().unwrap())
        .subject("Sending email with Rust - new")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(msg_final))
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

    // Send the email

    //  mailer.send(&email).expect("mail should have been sent");

    //   let mut a = 1;
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        // Ok(_) => a = 2,
        Err(e) => panic!("Could not send email: {:?}", e),
    } // end match
} // end mail
  //} // end all code
