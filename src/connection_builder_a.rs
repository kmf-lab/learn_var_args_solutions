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
}

#[derive(Debug)]
pub struct ConnectionBuilder {
    pub address: String,
    pub port: Option<u16>,
    pub protocol: Protocol,
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".into(),
            port: None,
            protocol: Protocol::Udp,
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

    pub(crate) fn build(self) -> Result<Connection, &'static str> {
        Ok(Connection {
            address: self.address,
            port: self.port.ok_or("port is required")?,
            protocol: self.protocol,
        })
    }
}

