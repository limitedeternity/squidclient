use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use clap::{crate_version, crate_authors, Arg, App};
use base64::{encode};

fn main() {
    let matches = App::new("squidclient")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Executes Squid proxy cachemgr commands")
        .arg(Arg::with_name("command")
                .help("A command to execute (e.g: \"menu\")")
                .required(true))
        .arg(Arg::with_name("host")
                .help("Squid proxy IP address")
                .short("h")
                .long("host")
                .required(true)
                .takes_value(true))
        .arg(Arg::with_name("port")
                .help("Squid proxy port")
                .short("p")
                .long("port")
                .default_value("3128")
                .takes_value(true))
        .arg(Arg::with_name("user")
                .help("Squid proxy cachemgr username")
                .short("u")
                .long("user")
                .takes_value(true))
        .arg(Arg::with_name("with_password")
                .help("Squid proxy cachemgr password")
                .short("w")
                .long("with_password")
                .takes_value(true))
        .get_matches();

    let mut stream = TcpStream::connect(format!("{}:{}", matches.value_of("host").unwrap(), matches.value_of("port").unwrap())).expect("Failed to connect to server");
    stream.set_read_timeout(Some(Duration::new(1, 0))).expect("Failed to set read timeout");

    let get = format!("GET cache_object://{}:{}/{} HTTP/1.1\r\n",
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap(), 
        matches.value_of("command").unwrap());

    let agent = format!("User-Agent: squidclient/{}\r\n", crate_version!());
    let host = format!("Host: {}\r\n", matches.value_of("host").unwrap());
    let accept = "Accept: */*\r\n";

    let auth = if matches.is_present("user") || matches.is_present("with_password") {
        format!("Authorization: Basic {}\r\n", encode([
                if let Some(user) = matches.value_of("user") { user } else { "" },
                ":",
                if let Some(password) = matches.value_of("with_password") { password } else { "" }
        ].concat()))
    } else {
        "\r\n".to_string()
    };

    stream.write(format!("{}{}{}{}{}\r\n", get, agent, host, accept, auth).as_bytes()).expect("Command write failed");

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        if let Ok(s) = line {
            println!("{}", s);
        } else {
            break;
        }
    }
}
