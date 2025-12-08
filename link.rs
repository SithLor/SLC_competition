#[repr(u8)]
#[derive(PartialEq)]
pub enum Protocol {
    Https,
    Http,
    Wss,
    Ws,
}
#[derive(PartialEq)]
struct Link {
    protocol: Protocol,
    host: String,
    port: u16,
    path: String,
}
pub trait LinkTrait {
    fn from_str(s: &str) -> Self;
    fn new(protocol: Protocol, host: &str, port: u16, path: &str) -> Self;
    fn set_protocol(&mut self, protocol: Protocol);
    fn set_host(&mut self, host: &str);
    fn set_port(&mut self, port: u16);
    fn set_path(&mut self, path: &str);
    fn get_protocol(&self) -> &Protocol;
    fn get_host(&self) -> &str;
    fn get_port(&self) -> u16;
    fn get_path(&self) -> &str;
    fn to_string(&self) -> String;
}
// protocol://host:port/path to Link { protocol, host, port, path }
impl Link {
    pub fn from(s: &str) -> Self {
        let mut parts: std::str::Split<'_, &str> = s.split("://");
        let protocol: Protocol = match parts.next() {
            Some("https") => Protocol::Https,
            Some("http") => Protocol::Http,
            Some("wss") => Protocol::Wss,
            Some("ws") => Protocol::Ws,
            _ => Protocol::Http,
        };
        let mut host_port_path = parts.next().unwrap().split("/");
        let host_port = host_port_path.next().unwrap();
        let mut host_port: std::str::Split<'_, &str> = host_port.split(":");
        let host: String = host_port.next().unwrap().to_string();
        let port: u16 = match host_port.next() {
            Some(port) => port.parse().unwrap(),
            None => match protocol {
                Protocol::Https => 443,
                Protocol::Http => 80,
                Protocol::Wss => 443,
                Protocol::Ws => 80,
            },
        };
        let path = host_port_path.collect();
        Link {
            protocol,
            host,
            port,
            path,
        }
    }
    pub fn new(protocol: Protocol, host: &str, port: u16, path: &str) -> Self {
        Link {
            protocol,
            host: host.to_string(),
            port,
            path: path.to_string(),
        }
    }
    pub fn set_protocol(&mut self, protocol: Protocol) {
        self.protocol = protocol;
    }
    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
    pub fn get_protocol(&self) -> &Protocol {
        &self.protocol
    }
    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}://{}:{}/{}",
            match self.protocol {
                Protocol::Https => "https",
                Protocol::Http => "http",
                Protocol::Wss => "wss",
                Protocol::Ws => "ws",
            },
            self.host,
            self.port,
            self.path
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_from_str() {

        //perftest 
        let mut data: Vec<std::time::Duration> = Vec::new();
        for _ in 0..1000000 {
            let time_start: std::time::Instant = std::time::Instant::now();


            let mut e = Link::from("https://example.com:8080/path");
            let mut _e;

            e.set_protocol(Protocol::Http);
            e.set_host("example.com");
            e.set_port(8080);
            e.set_path("path");
            _e = e.to_string();

            e.set_protocol(Protocol::Https);
            e.set_host("examples.com");
            e.set_port(443);
            e.set_path("paths");
            _e = e.to_string();

            e.set_protocol(Protocol::Wss);
            e.set_host("exampless.com");
            e.set_port(443);
            e.set_path("pathss");
            _e = e.to_string();

            e.set_protocol(Protocol::Ws);
            e.set_host("exampless.com");
            e.set_port(80);
            e.set_path("pathsss");
            _e = e.to_string();
            
            let __e: Link = Link::new(Protocol::Https, "example.com", 8080, "path");
            _e = __e.to_string();

            let endtime = time_start.elapsed();
            //push the time to data
            data.push(endtime);
        }
        //get the average time
        let mut sum: std::time::Duration = std::time::Duration::new(0, 0);
        for i in &data {
            sum += *i;
        }
        let average: std::time::Duration = sum / data.len() as u32;
        println!("Average time: {:?}", average);

    }
}

pub fn test(){
    let mut data: Vec<std::time::Duration> = Vec::new();
    for _ in 0..1000000 {
        let time_start: std::time::Instant = std::time::Instant::now();


        let mut e = Link::from("https://example.com:8080/path");
        let mut _e;

        e.set_protocol(Protocol::Http);
        e.set_host("example.com");
        e.set_port(8080);
        e.set_path("path");
        _e = e.to_string();

        e.set_protocol(Protocol::Https);
        e.set_host("examples.com");
        e.set_port(443);
        e.set_path("paths");
        _e = e.to_string();

        e.set_protocol(Protocol::Wss);
        e.set_host("exampless.com");
        e.set_port(443);
        e.set_path("pathss");
        _e = e.to_string();

        e.set_protocol(Protocol::Ws);
        e.set_host("exampless.com");
        e.set_port(80);
        e.set_path("pathsss");
        _e = e.to_string();
        
        let __e: Link = Link::new(Protocol::Https, "example.com", 8080, "path");
        _e = __e.to_string();

        let endtime = time_start.elapsed();
        //push the time to data
        data.push(endtime);
    }
    //get the average time
    let mut sum: std::time::Duration = std::time::Duration::new(0, 0);
    for i in &data {
        sum += *i;
    }
    let average: std::time::Duration = sum / data.len() as u32;
    println!("Average time: {:?}", average);

}