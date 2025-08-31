mod ringmembers;
mod templates;
use crate::ringmembers::{Ring, RingMember};
use crate::templates::homepage;
use clap::Parser;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use tiny_http::{Header, Method, Response, Server, ServerConfig};
use toml;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./ring.toml"))]
    config: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    members: HashMap<String, RingMember>,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    return 8080;
}
fn id_for_params(params: &str) -> Option<String> {
    for param in params.split("&") {
        if param.starts_with("id=") {
            return Some(param[3..].into());
        }
    }
    return None;
}

fn redirect_to(url: &String) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string("")
        .with_header(format!("location: {}", url).parse::<Header>().unwrap())
        .with_header(
            "cache-control: no-cache, no-store, no-transform"
                .parse::<Header>()
                .unwrap(),
        )
        .with_status_code(303)
}
fn main() {
    let cli_args = Args::parse();
    let config: Config = toml::from_str(
        read_to_string(Path::new(&cli_args.config))
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let ring = Ring::from_members(config.members);
    let server = Server::http(format!("127.0.0.0:{}", config.port)).unwrap();

    for request in server.incoming_requests() {
        let response = match request.method() {
            Method::Get => {
                let pos = request.url().find("?").unwrap_or(request.url().len());
                let route = &request.url()[..pos];
                match route {
                    "/next" => {
                        if let Some(id) = id_for_params(&request.url()[pos + 1..]) {
                            if let Some(member) = ring.next_id(id) {
                                let url = &member.url;
                                redirect_to(url)
                            } else {
                                redirect_to(&String::from("/"))
                            }
                        } else {
                            redirect_to(&String::from("/"))
                        }
                    }
                    "/previous" => {
                        if let Some(id) = id_for_params(&request.url()[pos + 1..]) {
                            if let Some(member) = ring.prev_id(id) {
                                let url = &member.url;
                                redirect_to(url)
                            } else {
                                redirect_to(&String::from("/"))
                            }
                        } else {
                            redirect_to(&String::from("/"))
                        }
                    }
                    "/random" => {
                        let mut rng = rand::rng();
                        let index = rng.random_range(0..ring.members.len());
                        if let Some(member) = ring.get_id_for_index(index) {
                            redirect_to(&member.url)
                        } else {
                            redirect_to(&String::from("/"))
                        }
                    }
                    _ => Response::from_string(homepage(&ring.members))
                        .with_status_code(200)
                        .with_header("content-type: text/html".parse::<Header>().unwrap()),
                }
            }
            _ => Response::from_string("").with_status_code(400),
        };

        request.respond(response);
    }
}
