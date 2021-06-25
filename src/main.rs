#[macro_use]
extern crate clap;

mod action;
mod hint;
mod notification;
mod notifications;
mod replace_file;
mod urgency;

use crate::action::Action;
use crate::hint::Hint;
use crate::notification::Notification;
use crate::notifications::{
    OrgFreedesktopNotifications, OrgFreedesktopNotificationsActionInvoked,
    OrgFreedesktopNotificationsNotificationClosed,
};
use crate::replace_file::ReplaceFile;
use crate::urgency::{Urgency, URGENCY_LEVELS};
use clap::{App, AppSettings, Arg, ErrorKind};
use dbus::arg::{RefArg, Variant};
use dbus::blocking::Connection;
use dbus::Message;
use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("notify-call")
        .set_term_width(0)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .version(clap::crate_version!())
        .about("Send desktop notifications and handle actions.")
        .help_short("?")
        .version_short("v")
        .arg(Arg::from_usage("-u, --urgency=[LEVEL]     'Notification urgency level.'").possible_values(&URGENCY_LEVELS))
        .arg(Arg::from_usage("-t, --expire-time=[TIME]  'Timeout in milliseconds at which the notification should automatically close. [default: -1]'"))
        .arg(Arg::from_usage("-a, --app-name=[APP_NAME] 'Name of application sending the notification.'"))
        .arg(Arg::from_usage("-i, --icon=[ICON]         'Icon filename or stock icon to display.'"))
        .arg(Arg::from_usage("-c, --category=[TYPE]     'Notification category (type indicator).'"))
        .arg(Arg::from_usage("-h, --hint=[TYPE:NAME:VALUE]... 'Extra data to a notification server. Valid types are int, double, string and byte.'").number_of_values(1))
        .arg(Arg::from_usage("-o, --action=[COMMAND:LABEL]... 'Notification action with a command to invoke.'").number_of_values(1))
        .arg(Arg::from_usage("-d, --default-action=[COMMAND]  'Default action (usually invoked by clicking notification).'"))
        .arg(Arg::from_usage("-r, --replace=[ID]        'Replace existing notification.'"))
        .arg(Arg::from_usage("-R, --replace-file=[FILE] 'Use file to store and replace notification ID.'"))
        .arg(Arg::from_usage("-p, --print-id            'Print notification ID.'"))
        .arg(Arg::from_usage("<SUMMARY>                 'Summary text (notification header).'"))
        .arg(Arg::from_usage("[BODY]                    'Detailed body text.'"))
        .get_matches();

    let summary = matches.value_of("SUMMARY").unwrap();
    let body = matches.value_of("BODY").unwrap_or("");
    let app_name = matches.value_of("app-name").unwrap_or("");
    let app_icon = matches.value_of("icon").unwrap_or("");
    let mut hints: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();

    if let Some(level) = matches.value_of("urgency") {
        let index = Urgency::from_str(level).unwrap() as u8;
        hints.insert("urgency", Variant(Box::new(index)));
    }

    let expire_timeout =
        value_t!(matches.value_of("expire-time"), i32).unwrap_or_else(|e| match e.kind {
            ErrorKind::ArgumentNotFound => -1,
            _ => e.exit(),
        });

    if let Some(category) = matches.value_of("category") {
        hints.insert("category", Variant(Box::new(category.to_string())));
    }

    if let Some(list) = matches.values_of("hint") {
        for tnv in list {
            let hint = Hint::from_str(tnv)?;
            hints.insert(hint.name, hint.value);
        }
    }

    let mut replaces_id =
        value_t!(matches.value_of("replace"), u32).unwrap_or_else(|e| match e.kind {
            ErrorKind::ArgumentNotFound => 0,
            _ => e.exit(),
        });

    let mut n11 = Notification::new();

    if let Some(cmd) = matches.value_of("default-action") {
        let action = Action::default_from_str(cmd);
        n11.add_action(String::from("default"), action);
    }

    if let Some(actions) = matches.values_of("action") {
        for (idx, cl) in actions.enumerate() {
            let action = Action::from_str(cl)?;
            n11.add_action(idx.to_string(), action);
        }
    }

    let replace_file = if let Some(filename) = matches.value_of("replace-file") {
        let file = ReplaceFile::new(filename);
        replaces_id = file.read_or(replaces_id)?;
        Some(file)
    } else {
        None
    };

    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        Duration::new(3, 0),
    );

    let nid = proxy.notify(
        app_name,
        replaces_id,
        app_icon,
        summary,
        body.replace("\\n", "\n").as_str(),
        n11.actions(),
        hints,
        expire_timeout,
    )?;

    if matches.is_present("print-id") {
        println!("{}", nid);
    }

    if let Some(file) = replace_file {
        if nid != replaces_id || !file.exists() {
            file.write(nid)?;
        }
    }

    if n11.has_actions() {
        proxy.match_signal(
            move |h: OrgFreedesktopNotificationsActionInvoked, _: &Connection, _: &Message| {
                if h.arg0 == nid {
                    n11.invoke_action(h.arg1);
                }
                true
            },
        )?;

        proxy.match_signal(
            move |h: OrgFreedesktopNotificationsNotificationClosed, _: &Connection, _: &Message| {
                if h.arg0 == nid {
                    exit(0);
                }
                true
            },
        )?;

        loop {
            conn.process(Duration::from_millis(1000))?;
        }
    }

    Ok(())
}
