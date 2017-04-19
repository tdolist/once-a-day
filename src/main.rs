
#[macro_use]
extern crate clap;
extern crate chrono;
extern crate lettre;
extern crate colored;
extern crate notify_rust;

use clap::App;
use colored::*;
use chrono::prelude::*;
use std::{thread, time};
use notify_rust::Notification;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};

#[cfg(windows)]
use std::process::exit;


static ONE_DAY: u64 = 86400;

fn main() {
    let yml = load_yaml!("cli.yml");
    let app = App::from_yaml(yml)
        .version(crate_version!())
        .get_matches();

    match app.subcommand() {
        ("run", Some(_)) => {
            println!("{} Run with notifications", "[Notice]".yellow().bold());
            execute(None);
        }
        ("server", Some(sub_m)) => {
            println!("{} Run as server", "[Notice]".yellow().bold());
            execute(Some(sub_m));
        }
        _ => println!("{}", app.usage()),
    }
}

fn execute(server_info: Option<&clap::ArgMatches>) {
    let notification = match server_info {
        Some(_) => false,
        None => true,
    };
    let sleep_duration = time::Duration::from_secs(ONE_DAY);
    loop {
        if notification {
            deliver_notifiaction();
        } else {
            send_mail(server_info.unwrap());
        }
        thread::sleep(sleep_duration);
    }
}

fn send_mail(args: &clap::ArgMatches) {
    let now = Local::now();
    let mail = EmailBuilder::new()
        .to(args.value_of("mail").unwrap())
        .from((args.value_of("mail").unwrap(), "Once a day"))
        .body("Be friendly and make a compliment today! :-)")
        .subject("Your compliment today")
        .build()
        .unwrap();
    let port = match args.value_of("port") {
        Some(x) => x.parse::<u16>().unwrap(),
        None => 587 as u16,
    };
    let mut sender = SmtpTransportBuilder::new((args.value_of("host").unwrap(), port))
        .unwrap()
        .credentials(args.value_of("user").unwrap(),
                     args.value_of("pass").unwrap())
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .connection_reuse(true)
        .build();
    match sender.send(mail) {
        Ok(_) => {
            println!("{} {}\tMail send to {}",
                     "[Log]   ".green().bold(),
                     now.format("%Y/%m/%d %H:%M:%S").to_string(),
                     args.value_of("mail").unwrap())
        }
        Err(x) => {
            println!("{} {}\tMail could not be sent because of {}",
                     "[Error]".red().bold(),
                     now.format("%Y/%m/%d %H:%M:%S").to_string(),
                     x)
        }
    }
}

#[cfg(all(unix))]
fn deliver_notifiaction() {
    let now = Local::now();
    match Notification::new()
        .summary("Once a day")
        .body("Be friendly and make a compliment today! 😊")
        .show() {
        Ok(_) => {
            println!("{} {}\tNotification delivered",
                     "[Log]   ".green().bold(),
                     now.format("%Y/%m/%d %H:%M:%S").to_string())
        }
        Err(x) => {
            println!("{} {}\tNotification could not be delivered because of {:?}",
                     "[Error]".red().bold(),
                     now.format("%Y/%m/%d %H:%M:%S").to_string(),
                     x)
        }
    }

}


#[cfg(windows)]
fn deliver_notifiaction() {
    let now = Local::now();
    println!("{} {}\tNo notification support for Windows right now.",
             "[Error]".red().bold(),
             now.format("%Y/%m/%d %H:%M:%S").to_string());
    exit(1);
}
