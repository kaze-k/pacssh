mod colors;
mod constants;
mod modules;
mod render;
mod system;
mod types;
mod utils;

use modules::Module;

fn main() {
    let info = system::info::Info::new();
    let ssh = system::ssh::Ssh::new();

    let username = info.username();
    let hostname = info.hostname();
    let ttyname = info.ttyname();

    let blank = modules::Blank::new();
    let login = modules::Login::new(ssh.is_ssh);
    let session = modules::Session::new(&username, &hostname);
    let tty = modules::Tty::new(&ttyname);
    let from = modules::From::new(&ssh.client_ip, &ssh.client_port);
    let to = modules::To::new(&ssh.server_ip, &ssh.server_port);

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

    let renderer = render::Renderer::new();

    renderer.render(&lines);
}
