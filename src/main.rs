mod colors;
mod constants;
mod modules;
mod render;
mod system;
mod types;
mod utils;

use modules::{Blank, From, Login, Module, Session, To, Tty};
use render::Renderer;
use system::{Info, Ssh};

fn main() {
    let info = Info::new();
    let ssh = Ssh::new();

    let username = info.username();
    let hostname = info.hostname();
    let ttyname = info.ttyname();

    let blank = Blank::new();
    let login = Login::new(ssh.is_ssh);
    let session = Session::new(&username, &hostname);
    let tty = Tty::new(&ttyname);
    let from = From::new(&ssh.client_ip, &ssh.client_port);
    let to = To::new(&ssh.server_ip, &ssh.server_port);

    let mut lines: Vec<Box<dyn Module>> = vec![
        Box::new(blank),
        Box::new(login),
        Box::new(session),
        Box::new(blank),
        Box::new(tty),
    ];

    if ssh.is_ssh {
        lines.push(Box::new(from));
        lines.push(Box::new(to));
    }

    lines.push(Box::new(blank));

    let renderer = Renderer::new();

    renderer.render(&lines);
}
