
#[macro_use]
extern crate clap;
#[macro_use]
extern crate duct;
extern crate regex;
extern crate chrono;
extern crate lettre;
extern crate colored;
extern crate notify_rust;
extern crate process_path;

use clap::Shell;
use colored::*;
use regex::Regex;
use chrono::prelude::*;
use std::{thread, time};
use notify_rust::Notification;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};

use std::process::exit;

mod cli;

fn main() {
    let re = Regex::new(r"([01]?\d):(\d{2})").unwrap();
    let app = cli::cli().get_matches();
    match app.subcommand() {
        ("run", Some(content)) => {
            let time = content.value_of("time").unwrap();
            if re.is_match(time) {
                println!("{} Run with notifications", "[Notice]".yellow().bold());
                let captures = re.captures(time).unwrap();
                execute(None,
                        captures.get(1).unwrap().as_str(),
                        captures.get(2).unwrap().as_str());
            } else {
                exit(1);
            }
        }
        ("server", Some(sub_m)) => {
            let time = sub_m.value_of("time").unwrap();
            if re.is_match(time) {
                println!("{} Run as server", "[Notice]".yellow().bold());
                let captures = re.captures(time).unwrap();
                execute(Some(sub_m),
                        captures.get(1).unwrap().as_str(),
                        captures.get(2).unwrap().as_str())
            } else {
                exit(1);
            }
        }
        ("screen", Some(sub_m)) => tmux_interaction(sub_m),
        ("completions", Some(sub_m)) => {
            if let Some(shell) = sub_m.value_of("shell") {
                cli::cli().gen_completions_to("oad",
                                              shell.parse::<Shell>().unwrap(),
                                              &mut std::io::stdout());
            }
        }
        _ => println!("{}", app.usage()),
    }
}

fn execute(server_info: Option<&clap::ArgMatches>, hour: &str, minute: &str) {
    let notification = match server_info {
        Some(_) => false,
        None => true,
    };
    let mut sleep_duration;
    let delta = time_delta(hour, minute, true);
    if delta.as_secs() > 0 {
        let now = Local::now();
        println!("{} {}\tFirst notification will be delivered in {} seconds",
                 "[Notice]".yellow().bold(),
                 now.format("%Y/%m/%d %H:%M:%S").to_string(),
                 delta.as_secs());
        thread::sleep(delta);
    }
    loop {
        sleep_duration = time_delta(hour, minute, false);
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
    let tmp_sender = match SmtpTransportBuilder::new((args.value_of("host").unwrap(), port)) {
        Ok(sender) => sender,
        Err(x) => {
            println!("{} {}\t{}",
                     "[Error]".red().bold(),
                     now.format("%Y/%m/%d %H:%M:%S").to_string(),
                     x);
            return;
        }
    };
    let mut sender = tmp_sender.credentials(args.value_of("user").unwrap(),
                     args.value_of("password").unwrap())
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

fn time_delta(hour: &str, minute: &str, first: bool) -> time::Duration {
    let trigger_base = if first {
        Local::today()
    } else {
        match Local::today().succ_opt() {
            Some(date) => date,
            None => {
                let now = Local::now();
                println!("{} {}\tThere is no tomorrow",
                         "[Error]".red().bold(),
                         now.format("%Y/%m/%d %H:%M:%S").to_string());
                exit(1);
            }
        }
    };
    let trigger_time = trigger_base.and_hms(hour.parse::<u32>().unwrap(),
                                            minute.parse::<u32>().unwrap(),
                                            0);
    let time_delta = trigger_time.timestamp() - Local::now().timestamp();
    if time_delta <= 0 {
        time::Duration::from_secs(0)
    } else {
        time::Duration::from_secs(time_delta as u64)
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
fn deliver_notifiaction(now: &date::Date) {
    let now = Local::now();
    println!("{} {}\tNo notification support for Windows right now.",
             "[Error]".red().bold(),
             now.format("%Y/%m/%d %H:%M:%S").to_string());
    exit(1);
}


fn tmux_interaction(commands: &clap::ArgMatches) {
    let executable = process_path::get_executable_path().unwrap().to_str().unwrap().to_string();
    match commands.subcommand() {
        ("run", Some(sub_m)) => {
            cmd!("screen",
                 "-dmS",
                 "once-a-day",
                 executable,
                 "run",
                 sub_m.value_of("time").unwrap())
                .run()
                .unwrap();
        }
        ("server", Some(sub_m)) => {
            cmd!("screen",
                 "-dmS",
                 "once-a-day",
                 executable,
                 "server",
                 sub_m.value_of("mail").unwrap(),
                 sub_m.value_of("host").unwrap(),
                 sub_m.value_of("user").unwrap(),
                 sub_m.value_of("password").unwrap(),
                 sub_m.value_of("time").unwrap(),
                 sub_m.value_of("port").unwrap_or("587"))
                .run()
                .unwrap();
        }
        ("status", _) => {
            match cmd!("screen", "-ls").unchecked().pipe(cmd!("grep", "-q", "once-a-day")).run() {
                Ok(_) => println!("Status of Once-a-day: {}", "running".green().bold()),
                Err(_) => println!("Status of Once-a-day: {}", "not running".red().bold()),
            };

        }
        ("stop", _) => {
            match cmd!("screen", "-X", "-S", "once-a-day", "quit").stdout_null().run() {
                Ok(_) => println!("Once-a-day was stopped"),
                Err(_) => println!("Once-a-day was not running"),
            }
        }
        _ => println!("{}", commands.usage()),
    }
}
