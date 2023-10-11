#!/usr/bin/env rust-script

use std::io::Read;
use std::process::Command;

fn main() {
    let mut cmd_focus_next = Command::new("zellij");
    cmd_focus_next.arg("action").arg("focus-next-pane");
    let mut cmd_write = Command::new("zellij");
    cmd_write.arg("action").arg("write");
    let mut input =  Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_end(&mut input);
    for byte in input {
        cmd_write.arg(format!("{}", byte));
    }
    let _ = cmd_focus_next.status();
    let _ = cmd_write.status();
    let _ = cmd_focus_next.status();
}

