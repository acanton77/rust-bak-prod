// RS_BAK_PROD
// RS_BAK_PROD
// RS_BAK_PROD

#![allow(warnings)]

use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::US::Pacific;
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
    // ========

    //==========

    // CREATE THE VEC FROM THE SWITCH FILE (call function)

    let mut vec_switch_file = lines_from_file(SWITCH_FILE).expect("Could not load lines");

    //*** APPEND DAILY ITEMS TO VEC_SWITCH_FILE

    vec_switch_file.push("linode".to_string());
    vec_switch_file.push("ac_addressbook".to_string());
    vec_switch_file.push("chk_espo_ver".to_string());

    //*** SORT THE VECTOR

    vec_switch_file.sort();

    //*** DE-DUP THE VECTOR

    vec_switch_file.dedup();

    //====write messg========================

    message_data = "vec_switch and appends DONE".to_string();
    write_msg(&mut msg_vec, message_data);

    //***********************LOOP *********************************
    //***********************LOOP *********************************
    //***********************LOOP *********************************

    for line in &vec_switch_file {
        if line == "1test" {
            //println!("we got 1test and break");
            //break;
        }

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

        if line == "piwigo" {
            // println!("we got piwigo");
        }

        if line == "alicegift" {
            // println!("we will do alicegift");
            // alicegift(&mut msg1);
        }

        if line == "ancins" {
            zip_in_file = "/usr/home/ancnet1/public_html/ancins.com".to_string();
            zip_out_file_name = "ancins.com-rs-".to_string();
            message_data = "ancins backup is DONE".to_string();
            rsync_dir = "web-backup-ancins".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        }

        if line == "k6aai" {
            zip_in_file = "/usr/home/ancnet1/public_html/k6aai.net".to_string();
            // zip_out_file = "/usr/home/ancnet1/rs_bak_prod/bak_files/k6aai.net-rs-".to_string();
            zip_out_file_name = "k6aai.net-rs-".to_string();
            message_data = "k6aai backup is DONE".to_string();
            rsync_dir = "web-backup-k6aai".to_string();
            bak_bootstrap(zip_in_file, zip_out_file_name, &rsync_dir);
            write_msg(&mut msg_vec, message_data);
        }

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

        if line == "jack" {
            // println!("we got jack");
        }
    } //for loop

    //*********************** END LOOP *********************************
    //*********************** END LOOP *********************************

    // Send the mail

    send_mail(&mut msg_vec);

    //truncate switch file

    let file = OpenOptions::new().write(true).truncate(true).open(SWITCH_FILE);

    println!("---END OF CODE---");

    // ****************** FUNCTIONS *************************************************
    // ****************** FUNCTIONS *************************************************
    // ****************** FUNCTIONS *************************************************

    //=== FUNCTION: BOOTSTRAP ================================================
    //=== FUNCTION: BOOTSTRAP ================================================

    fn bak_bootstrap(
        mut zip_in_file: String,
        mut zip_out_file_name: String,
        mut rsync_dir: &String,
    ) {
        let mut zip_out_file: String =
            format!("{}{}", HOME_FILE_DIR.to_string(), zip_out_file_name);
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

        let mut dump_out_file: String =
            format!("{}{}", HOME_FILE_DIR.to_string(), dump_out_file_name);

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

    fn send_mail(msg_vec: &mut Vec<(String, String, String)>) {
        let mut msg_final = String::new();

        msg_final.push_str("this is the first line of the message\n");
        msg_final.push_str("this is the 2nd line of the message\n");
        msg_final.push_str("this is the 3rd line of the message\n");

        msg_final.push_str("\n");

        // get the msg file and pop in line numbrers

        let mut num = 0;
        let mut newvalue: String;
        let mut from_orig: String;

        for line in msg_vec {
            num = num + 1;
            let mut num2: &str = &num.to_string();
            let newvalue = format!("   {}--  {}  {}\n", num2, line.1, line.2);

            msg_final.push_str(&newvalue);
        } // end for line read loop

        msg_final.push_str("\n");
        msg_final.push_str("this is after the messages 1 \n");
        msg_final.push_str("this is the 2nd line after the messages\n");
        msg_final.push_str("this is the 3rd and LAST line after the message\n");
        //msg_final.push_str("-------- END ----------\n");

        // println!("========== START MSG FILE ==========");

        // println!("{}", msg_final);
        // println!("========== END MSG FILE ==========");

        // Send the mail

        let email = Message::builder()
            .from("Pair-Rust-VPS <ac99@answer123.com>".parse().unwrap())
            .to("Receiver <ac99@answer123.com>".parse().unwrap())
            .subject("Sending email with Rust - new")
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
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        } // end match
    } // end mail
} // end all code
