use std::env::{args, var};
use std::process::Command;

fn main() {
    let mut arg_idx = 0;

    let mut domain: String = String::from("");
    let mut author: String = String::from("");
    let mut repo: String = String::from("");

    let mut ssh: bool = false;

    for argument in args() {
        match arg_idx {
            0 => (),
            1 => domain = argument,
            2 => author = argument,
            3 => repo = argument,
            4 => {
                if argument == String::from("ssh") {
                    ssh = true;
                }
            }
            _ => println!("---> Too many arguments! Ignoring extras..."),
        }

        arg_idx += 1;
    }

    let check = String::from("");
    if domain == check || author == check || repo == check {
        println!("Make sure to pass all three args!\nionize domain author repo")
    } else {
        clone_into_ionized_path(domain, author, repo, ssh)
    }
}

fn clone_into_ionized_path(domain: String, author: String, repo: String, ssh: bool) {
    let ionized_path = find_ionized_path();
    let ionized_src_path = format!("{}/src", ionized_path);

    let from = clone_protocol(&domain, &author, &repo, &ssh);
    let into = clone_destination(&ionized_src_path, &domain, &author, &repo);

    println!("---> Ionize is cloning: {}\n---> Into: {}", from, into);

    Command::new("git")
        .arg("clone")
        .arg(from)
        .arg(into)
        .output()
        .expect("Something went wrong trying to fetch the repo");

    println!("---> {} has been fetched successfully!", repo);
}

fn find_ionized_path() -> String {
    let mut home: &str = "HOME";

    if cfg!(target_os = "windows") {
        home = "USERPROFILE";
    }

    let mut ionized_path = var("IONIZED_PATH").unwrap();

    if ionized_path == "" {
        ionized_path = format!("{}/rustlang", home);
    }

    ionized_path
}

fn clone_protocol(domain: &String, author: &String, repo: &String, ssh: &bool) -> String {
    let mut protocol = format!("git@{}:{}/{}", domain, author, repo);

    if !ssh {
        protocol = format!("https://{}/{}/{}", domain, author, repo);
        println!("---> Protocol is: https");
    } else {
        println!("---> Protocol is: ssh");
    }

    protocol
}

fn clone_destination(
    ionized_src_path: &String,
    domain: &String,
    author: &String,
    repo: &String,
) -> String {
    format!("{}/{}/{}/{}", ionized_src_path, &domain, &author, &repo)
}