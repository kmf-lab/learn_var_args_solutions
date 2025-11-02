// main.rs

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Connection {
    pub address: String,
    pub port: u16,
    pub protocol: Protocol,
    pub encryption: bool,
}

#[derive(Debug, Clone)]
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
    /// Immutable setter for address — returns new builder with given value
    pub(crate) fn address(&self, address: impl Into<String>) -> Self {
        let mut new = self.clone();
        new.address = address.into();
        new
    }

    /// Immutable setter for protocol — returns new builder with given value
    pub(crate) fn protocol(&self, protocol: Protocol) -> Self {
        let mut new = self.clone();
        new.protocol = protocol;
        new
    }

    /// Immutable setter for port — returns new builder with given value
    pub(crate) fn port(&self, port: u16) -> Self {
        let mut new = self.clone();
        new.port = Some(port);
        new
    }

    pub(crate) fn encryption(&self, encryption: bool) -> Self {
        let mut new = self.clone();
        new.encryption = encryption;
        new
    }

    /// Build the final `Connection`
    pub(crate) fn build(&self) -> Result<Connection, &'static str> {
        Ok(Connection {
            address: self.address.clone(),
            port: self.port.ok_or("port is required")?,
            protocol: self.protocol.clone(),
            encryption: self.encryption,
        })
    }
}


pub(crate) fn use_connection(c: &Connection) -> String {
    format!("{}:{}", c.address, c.port)
}




