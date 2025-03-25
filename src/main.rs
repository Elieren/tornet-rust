use clap::{Arg, Command};
use colored::Colorize;
use reqwest::blocking::Client;
use reqwest::Proxy;
use signal_hook::{consts::signal::*, iterator::Signals};
use std::process::Stdio;
use std::{process::Command as ShellCommand, thread, time::Duration};

fn install_tor() {
    let output_one = ShellCommand::new("which")
        .arg("tor")
        .output()
        .expect("Error");

    if !output_one.status.success() {
        let output_two = ShellCommand::new("apt")
            .arg("update")
            .output()
            .expect("Error");

        if !output_two.status.success() {
            println!("{}", "Failed to update package list.".red());
            return;
        }
        let output_three = ShellCommand::new("apt")
            .arg("install")
            .arg("tor")
            .arg("-y")
            .output()
            .expect("Error");

        if output_three.status.success() {
            println!("{}", "Tor installed successfully.".green());
        } else {
            println!("{}", "Failed to install Tor.".red());
        }
    } else {
        println!("{}", "Tor is already installed.".yellow());
    }
}

fn is_tor_installed() -> bool {
    let output = ShellCommand::new("which")
        .arg("tor")
        .output()
        .expect("Error check tor");

    if output.status.success() {
        return true;
    } else {
        return false;
    }
}

fn initialize_environment() {
    let output = ShellCommand::new("service")
        .arg("tor")
        .arg("start")
        .status()
        .expect("Error starting tor");

    if !output.success() {
        println!("{}", "Error starting tor".red());
    }

    print_start_message();
}

fn print_start_message() {
    println!(
        "{}",
        "Tor service started. Please wait a minute for Tor to connect.".yellow()
    );
    println!(
        "{}",
        "Make sure to configure your browser to use Tor for anonymity.".yellow()
    );
}

fn ma_ip() -> Option<String> {
    if is_tor_running() {
        return ma_ip_tor();
    } else {
        return ma_ip_normal();
    }
}

fn is_tor_running() -> bool {
    let output = ShellCommand::new("pgrep")
        .arg("-x")
        .arg("tor")
        .stdout(Stdio::null())
        .status()
        .expect("ERROR");

    return output.success();
}

fn ma_ip_tor() -> Option<String> {
    let url: &str = "https://api.ipify.org";
    let proxy = Proxy::all("socks5://127.0.0.1:9050");

    let client = Client::builder()
        .proxy(proxy.expect("Error connect to proxy"))
        .build()
        .expect("Error connect to proxy");

    match client.get(url).send() {
        Ok(response) if response.status().is_success() => response.text().ok(),
        _ => {
            println!(
                "{}",
                "Having trouble connecting to the Tor network. Wait a minute.".yellow()
            );
            None
        }
    }
}

fn ma_ip_normal() -> Option<String> {
    let url: &str = "https://api.ipify.org";

    let client = Client::builder().build().expect("Error");

    match client.get(url).send() {
        Ok(response) if response.status().is_success() => response.text().ok(),
        _ => {
            println!(
                "{}",
                "Having trouble fetching the IP address. Please check your internet connection."
                    .yellow()
            );
            None
        }
    }
}

fn change_ip() -> Option<String> {
    let output = ShellCommand::new("service")
        .arg("tor")
        .arg("reload")
        .status()
        .expect("Error reload tor");

    if !output.success() {
        println!("{}", "Error reload tor".red());
    }

    return ma_ip();
}

fn change_ip_repeatedly(interval: u64, count: u64) {
    if count == 0 {
        loop {
            thread::sleep(Duration::from_secs(interval));
            if let Some(new_ip) = change_ip() {
                print_ip(&new_ip);
            }
        }
    } else {
        for _ in 0..count {
            thread::sleep(Duration::from_secs(interval));
            if let Some(new_ip) = change_ip() {
                print_ip(&new_ip);
            }
        }
    }
}

fn print_ip(ip: &str) {
    let text: String = format!("Your IP has been changed to {}", ip);
    println!("{}", text.green());
}

fn auto_fix() {
    install_tor();
}

fn stop_services() {
    let output_one = ShellCommand::new("pkill")
        .arg("-f")
        .arg("tor")
        .status()
        .expect("Error");

    if !output_one.success() {
        println!("{}", "Error stop tor!".red());
    }

    let output_two = ShellCommand::new("pkill")
        .arg("-f")
        .arg("tornet")
        .status()
        .expect("Error");

    if !output_two.success() {
        println!("{}", "Error stop tornet!".red());
    }

    println!("{}", "Tor service and tornet processes stopped.".yellow());
}

fn check_internet_connection() {
    let client = Client::builder().build().expect("Error create client");

    let status = client.get("https://google.com").send().expect("Error");

    if !status.status().is_success() {
        println!(
            "{}",
            "Internet connection lost. Please check your internet connection.".red()
        );
        println!("{}", "\n[!] Stopping the program...".yellow());
        println!("{}", "\n[!] Program stopped.".green());
        std::process::exit(0)
    }
}

fn main() {
    let logo = r#"
 ████████╗ ██████╗ ██████╗ ███╗   ██╗███████╗████████╗
 ╚══██╔══╝██╔═══██╗██╔══██╗████╗  ██║██╔════╝╚══██╔══╝
    ██║   ██║   ██║██████╔╝██╔██╗ ██║█████╗     ██║
    ██║   ██║   ██║██╔══██╗██║╚██╗██║██╔══╝     ██║
    ██║   ╚██████╔╝██║  ██║██║ ╚████║███████╗   ██║
    ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝   ╚═╝
                     @Elieren
        "#;

    check_internet_connection();

    let matches = Command::new("TorNet")
        .version("2.0.0")
        .about("Automate IP address changes using Tor")
        .arg(
            Arg::new("interval")
                .long("interval")
                .default_value("60")
                .help("Time in seconds between IP changes"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .default_value("10")
                .help("Number of times to change the IP"),
        )
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Display the current IP address and exit"),
        )
        .arg(
            Arg::new("auto-fix")
                .long("auto-fix")
                .help("Automatically fix issues"),
        )
        .arg(Arg::new("stop").long("stop").help("Stop all Tor services"))
        .get_matches();

    let mut signals = Signals::new(&[SIGINT, SIGQUIT]).expect("Ошибка при настройке сигналов");
    thread::spawn(move || {
        for _ in signals.forever() {
            println!("{}", "\n[!] Stopping the program...".yellow());
            println!("{}", "\n[!] Program stopped.".green());
            std::process::exit(0);
        }
    });

    if matches.contains_id("ip") {
        if let Some(ip) = ma_ip() {
            print_ip(&ip);
        }
        return;
    }

    if !is_tor_installed() {
        println!(
            "{}",
            "Tor is not installed. Please install Tor and try again.".red()
        );
        return;
    }

    if matches.contains_id("auto-fix") {
        auto_fix();
        println!("{}", "Auto-fix complete.".green());
        return;
    }

    if matches.contains_id("stop") {
        stop_services();
        return;
    }

    println!("{}", logo.green());
    initialize_environment();

    let interval = matches
        .get_one::<String>("interval")
        .unwrap_or(&"60".to_string()) // Берём значение по умолчанию
        .parse::<u64>()
        .expect("Interval must be a valid number");

    let count = matches
        .get_one::<String>("count")
        .unwrap_or(&"10".to_string()) // Берём значение по умолчанию
        .parse::<u64>()
        .expect("Count must be a valid number");

    change_ip_repeatedly(interval, count);
}
