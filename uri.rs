//uri rfc3986 impl

use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug, Clone,PartialEq)]
pub enum Host {
    IP(IpAddr),
    RegisteredName(String),
}

#[derive(Debug)]
pub struct URI {
    pub scheme: String,
    pub user_info: Option<String>,
    pub host: Host,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}
impl FromStr for URI {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("://").collect();
        let scheme = parts[0].to_string();

        let parts: Vec<&str> = parts[1].split("@").collect();
        let user_info: Option<String> = parts.get(0).map(|s| s.to_string());

        let parts: Vec<&str> = parts[1].split(":").collect();
        let host_part = parts[0];
        let host = if host_part.starts_with('[') && host_part.ends_with(']') {
            // This is an IP literal
            let ip_literal = &host_part[1..host_part.len() - 1];
            let ip = IpAddr::from_str(ip_literal)?;
            Host::IP(ip)
        } else if host_part.parse::<Ipv4Addr>().is_ok() {
            // This is an IPv4 address
            let ip = IpAddr::V4(host_part.parse::<Ipv4Addr>()?);
            Host::IP(ip)
        } else {
            // This is a registered name
            Host::RegisteredName(host_part.to_string())
        };
        let parts: Vec<&str> = parts[1].split("/").collect();
        let port: Option<u16> = parts[0].parse::<u16>().ok();

        let parts: Vec<&str> = parts[1].split("?").collect();
        let path = parts[0].to_string();

        let parts: Vec<&str> = parts.get(1).unwrap_or(&"").split("#").collect();
        let query = parts.get(0).map(|s| s.to_string());

        let fragment = parts.get(1).map(|s| s.to_string());

        Ok(URI {
            scheme,
            user_info,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}
impl ToString for URI {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.scheme);
        s.push_str("://");
        if let Some(user_info) = &self.user_info {
            s.push_str(&user_info);
            s.push_str("@");
        }
        match &self.host {
            Host::IP(ip) => s.push_str(&ip.to_string()),
            Host::RegisteredName(name) => s.push_str(&name),
        }
        if let Some(port) = self.port {
            s.push_str(":");
            s.push_str(&port.to_string());
        }
        s.push_str(&self.path);
        if let Some(query) = &self.query {
            s.push_str("?");
            s.push_str(&query);
        }
        if let Some(fragment) = &self.fragment {
            s.push_str("#");
            s.push_str(&fragment);
        }
        s
    }
}

pub trait UriGetterSetter {
    fn get_scheme(&self) -> String;
    fn set_scheme(&mut self, scheme: String);
    fn get_user_info(&self) -> Option<String>;
    fn set_user_info(&mut self, user_info: Option<String>);
    fn get_host(&self) -> Host;
    fn set_host(&mut self, host: Host);
    fn get_port(&self) -> Option<u16>;
    fn set_port(&mut self, port: Option<u16>);
    fn get_path(&self) -> String;
    fn set_path(&mut self, path: String);
    fn get_query(&self) -> Option<String>;
    fn set_query(&mut self, query: Option<String>);
    fn get_fragment(&self) -> Option<String>;
    fn set_fragment(&mut self, fragment: Option<String>);
}
impl UriGetterSetter for URI {
    fn get_scheme(&self) -> String {
        self.scheme.clone()
    }
    fn set_scheme(&mut self, scheme: String) {
        self.scheme = scheme;
    }
    fn get_user_info(&self) -> Option<String> {
        self.user_info.clone()
    }
    fn set_user_info(&mut self, user_info: Option<String>) {
        self.user_info = user_info;
    }
    fn get_host(&self) -> Host {
        self.host.clone()
    }
    fn set_host(&mut self, host: Host) {
        self.host = host;
    }
    fn get_port(&self) -> Option<u16> {
        self.port
    }
    fn set_port(&mut self, port: Option<u16>) {
        self.port = port;
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
    fn set_path(&mut self, path: String) {
        self.path = path;
    }
    fn get_query(&self) -> Option<String> {
        self.query.clone()
    }
    fn set_query(&mut self, query: Option<String>) {
        self.query = query;
    }
    fn get_fragment(&self) -> Option<String> {
        self.fragment.clone()
    }
    fn set_fragment(&mut self, fragment: Option<String>) {
        self.fragment = fragment;
    }
}

