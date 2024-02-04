// rs_bak_prod
// rs_bak_prod
// rs_bak_prod

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

const RSYNC_ADDRESS: &str = "fm1364@fm1364.rsync.net:";
const SWITCH_FILE: &str = "/usr/home/ancnet1/rs_bak/bak_files/switch_file.txt";

fn main() {
    let mut msg_vec: Vec<(String, String, String)> = Vec::new();
    let mut message_data: String = "".to_string();

    //*** CREATE VEC FROM THE SWITCH FILE (call function)

    let mut vec_switch_file = lines_from_file(SWITCH_FILE).expect("Could not load lines");

    //     for line in &vec_switch_file {
    //         println!("vec_switch item is: {:?}", line);
    //
    //     } // end of for

    //*** APPEND DAILY ITEMS TO VEC_SWITCH_FILE

    vec_switch_file.push("linode".to_string());
    vec_switch_file.push("ac_addressbook".to_string());
    vec_switch_file.push("chk_espo_ver".to_string());

    //     println!("---after append---");
    //
    //   for line in &vec_switch_file {
    //         println!("vec_switch append is: {:?}", line);
    //     } // end for

    //*** SORT THE VECTOR

    vec_switch_file.sort();

    //*** DE-DUP THE VECTOR

    vec_switch_file.dedup();

    //====write messg========================

    //let mut message  = (("222".to_string(), "Feb 26 02:02:02".to_string(), "vec_switch and appends donw".to_string()));
    message_data = "vec_switch and appends DONE".to_string();
    write_msg(&mut msg_vec, message_data);

    // A FEW MORE RECORDS

    message_data = "alice WP is DONE".to_string();
    write_msg(&mut msg_vec, message_data);

    message_data = "ancins backup is DONE".to_string();
    write_msg(&mut msg_vec, message_data);

    for line in &msg_vec {
        println!(" after call to write: {}  {}  {}", line.0, line.1, line.2);
    }

    send_mail(&mut msg_vec);

    println!("---END OF CODE---");

    // ****************** FUNCTIONS *************************************************
    // ****************** FUNCTIONS *************************************************
    // ****************** FUNCTIONS *************************************************

    //=== FUNCTION: CURRENT DATE ================================================

    fn get_current_date() -> String {
        let current_local: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
        //  println!("pacific_now = {}", current_local);

        //let current_local: DateTime<Local> = Local::now();

        // println!("curent local: {}", current_local);
        let msg_date1 = current_local.format("%Y-%m-%d %H:%M:%S:%3f %Z");

        //println!(" message date: {}", msg_date1);

        msg_date1.to_string()
    } // end current date

    //=== FUNCTION: TIME STAMP ================================================

    fn get_timestamp() -> String {
        let milliseconds_timestamp: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // println!("mile is: {}", milliseconds_timestamp);

        let timestamp_string: String;
        timestamp_string = milliseconds_timestamp.to_string();

        timestamp_string // return
    } // end time stamp

    //=== FUNCTION: CREATE VEC FROM SWITCH FILE================================================

    fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
        BufReader::new(File::open(filename)?).lines().collect()
        //lines for vec 88
    } // end create vec

    // FUNCTION: WRITE MESSAGE

    fn write_msg(msg_vec: &mut Vec<(String, String, String)>, message_data: String) {
        thread::sleep(Duration::from_millis(100));
        let msg_date: String = get_current_date();

        let time_stamp = get_timestamp();

        //         msg_vec.push((
        //             "299".to_string(),
        //             "June 26 04:02:06".to_string(),
        //             message_data,
        //         ));

        msg_vec.push((time_stamp.to_string(), msg_date.to_string(), message_data));

        //for line in msg_vec {
        //  println!(" in function tupple is: {} {}  {}", line.0, line.1, line.2);
        //}
    } // end write msg

    // FUNCTION: SEND MAIL

    fn send_mail(msg_vec: &mut Vec<(String, String, String)>) {
        // fn send_mail(msg_vec: &mut Vec<(String, String, String))> {
        let mut msg_final = String::new();

        //let mut msg = String::new();
        //let mut msg = String::from( "this is first line\nand this is seoncd\n");
        msg_final.push_str("this is the first line of the message\n");
        msg_final.push_str("this is the 2nd line of the message\n");
        msg_final.push_str("this is the 3rd line of the message\n");

        msg_final.push_str("\n");

        // get the msg file and pop in line numbrers

        let mut num = 0;
        let mut newvalue: String;
        //let mut fromcolors:String;
        let mut from_orig: String;

        for line in msg_vec {
            num = num + 1;
            let mut num2: &str = &num.to_string();
            let newvalue = format!("   {}--  {}  {}\n", num2, line.1, line.2);

            // print!("value msg1 with line num is: {}", &newvalue);
            msg_final.push_str(&newvalue);
        } // end for loop

        println!("*** final ***\n");
        for line in msg_final.lines() {
            println!("{}", line);
        }
        //return;

        msg_final.push_str("\n");
        msg_final.push_str("this is after the messages 1 \n");
        msg_final.push_str("this is the 2nd line after the messages\n");
        msg_final.push_str("this is the 3rd line after the message\n");
        msg_final.push_str("-------- END ----------\n");

        println!("========== START MSG FILE ==========");

        println!("{}", msg_final);
        println!("========== END MSG FILE ==========");

        // Send the mail

        let email = Message::builder()
            .from("Pair-Rust-VPS <ac99@answer123.com>".parse().unwrap())
            .to("Receiver <ac99@answer123.com>".parse().unwrap())
            .subject("Sending email with Rust - new")
            //.body(String::from("This is my first33333 email\n"))
            .body(String::from(msg_final))
            .unwrap();

        let creds = Credentials::new(
            "smtp2@mailanc2.net".to_string(),
            "tek7iah77mail-".to_string(),
        );
        //let creds = Credentials::new("smtp2@mailanc2.net".to_string(), /usr/home/ancnet1/.ssh/id_ed25519);

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("ancnet1.mail.pairserver.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    } // end mail
} // end all code
