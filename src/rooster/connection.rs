pub struct ConnectionData { 
    pub u_name: String,
    pub u_passwd: String,
    pub host: String,
    pub port: u16
}

impl ConnectionData { 
    pub fn new(u_name: String, u_passwd: String, host: Option<String>, port: Option<u16>) -> ConnectionData {
        return ConnectionData {
            u_name: u_name,
            u_passwd: u_passwd,
            host: host.unwrap_or(String::from("localhost") ),
            port: port.unwrap_or(3306),
        };
    }


    pub fn toUrl(&self) -> String { 
        return format!("mysql://{}:{}@{}:{}/test", self.u_name, self.u_passwd, self.host, self.port);
    }
}

