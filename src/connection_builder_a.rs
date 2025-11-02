#[derive(Debug)]
#[allow(dead_code)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Connection {
    pub address: String,
    pub port: u16,
    pub protocol: Protocol,
    pub encryption: bool,
}

#[derive(Debug)]
pub struct ConnectionBuilder {
    pub address: String,
    pub port: Option<u16>,
    pub protocol: Protocol,
    pub encryption: bool,
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".into(),
            port: None,
            protocol: Protocol::Udp,
            encryption: false,
        }
    }
}

#[allow(dead_code)]
impl ConnectionBuilder {
    pub(crate) fn address(mut self, address: impl Into<String>) -> Self {
        self.address = address.into();
        self
    }

    pub(crate) fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub(crate) fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub(crate) fn encryption(mut self, enabled: bool) -> Self {
        self.encryption = enabled;
        self
    }

    // alternate return with validation
    //pub(crate) fn build_secure(self) -> Result<SecureConnection, &'static str> 
    
    pub(crate) fn build(self) -> Result<Connection, &'static str> {
        Ok(Connection {
            address: self.address,
            port: self.port.ok_or("port is required")?,
            protocol: self.protocol,
            encryption: self.encryption,
        })
    }
}



// let conn = maybe_port
// .map(|p| builder.port(p))
// .unwrap_or(builder.port(8080))
// .build()?;


//we can combine macros with the builder pattern
#[macro_export]
macro_rules! connect {
    // (address, port)
    ($addr:expr, $port:expr) => {
        ConnectionBuilder::default()
            .address($addr)
            .port($port)
            .build()
    };
    // (address, port, encryption)
    ($addr:expr, $port:expr, $enc:expr) => {
        ConnectionBuilder::default()
            .address($addr)
            .port($port)
            .encryption($enc)
            .build()
    };
}

#[allow(dead_code)]
pub(crate) fn use_connection(conn: &Connection) {
    println!("{conn:?}");
}

//  hybrid macro + builder
// let a = connect!("10.0.0.1", 8080).unwrap();
// let b = connect!("10.0.0.1", 443, true).unwrap();
// println!("{:?}\n{:?}", a, b);
// 
// 👉 Teaching point:
// 
// 
// “Macros can simulate overloads by pattern‑matching argument counts,
// 
// but we lose type guidance, IDE completion, and compile clarity.
// 
// Builders, enums, and traits express intent far better.”