use std::process::Command;
use regex::Regex;
extern crate notify_rust;
use notify_rust::Notification;

fn main() {

    let re = Regex::new(r"No new mail.").unwrap();
    
    let check = Command::new("ssh")
        .arg("mail@mail-server")
        .arg("notmuch new")
        .output()
        .expect("Failed to SSH to mail-server.");

    if !re.is_match(&String::from_utf8_lossy(&check.stdout)) {

	Notification::new()
        .summary("New mail found on mail-server.")
	.body(&String::from_utf8_lossy(&check.stdout))
        .icon("mail-unread")
        .show().unwrap();
	
	Command::new("aplay")
	    .arg("/home/tatsu/Programming/notmuch_notifications/media/spock_mail.wav")
	    .spawn()
            .expect("Failed to play notification sound.");
    }
}
