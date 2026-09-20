use std::env;

pub struct Ssh {
    pub is_ssh: bool,
    pub client_ip: String,
    pub client_port: String,
    pub server_ip: String,
    pub server_port: String,
}

impl Ssh {
    pub fn new() -> Self {
        let Some(connection) = env::var_os("SSH_CONNECTION") else {
            return Self {
                is_ssh: false,
                client_ip: "unknown".to_string(),
                client_port: "unknown".to_string(),
                server_ip: "unknown".to_string(),
                server_port: "unknown".to_string(),
            };
        };

        let connection = connection.to_string_lossy().into_owned();
        let mut parts = connection.split_whitespace();

        let client_ip = parts
            .next()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let client_port = parts
            .next()
            .map(|port| port.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let server_ip = parts
            .next()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let server_port = parts
            .next()
            .map(|port| port.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            is_ssh: true,
            client_ip,
            client_port,
            server_ip,
            server_port,
        }
    }
}
