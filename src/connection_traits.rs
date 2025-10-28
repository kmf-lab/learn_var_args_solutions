// ------------------------------------------------------------
// connection_traits.rs
// ------------------------------------------------------------

// The trait every connection type must implement.
pub trait Connectable: std::fmt::Debug {
    fn describe(&self) -> String;
    fn connect(&self);
}

// ---------------------------------------------------------------------
// Concrete connection types
// ---------------------------------------------------------------------

#[derive(Debug)]
pub struct TcpConnection {
    pub address: String,
    pub port: u16,
    pub encryption: bool,
}

impl Connectable for TcpConnection {
    fn describe(&self) -> String {
        format!(
            "tcp://{}:{} (encryption: {})",
            self.address, self.port, self.encryption
        )
    }

    fn connect(&self) {
        println!(
            "[TCP] Connecting to {}:{} (encryption: {})",
            self.address, self.port, self.encryption
        );
    }
}

#[derive(Debug)]
pub struct UdpConnection {
    pub address: String,
    pub port: u16,
}

impl Connectable for UdpConnection {
    fn describe(&self) -> String {
        format!("udp://{}:{}", self.address, self.port)
    }

    fn connect(&self) {
        println!("[UDP] Connecting to {}:{}", self.address, self.port);
    }
}

#[derive(Debug)]
pub struct LocalHostConnection {
    pub port: u16,
}

impl Connectable for LocalHostConnection {
    fn describe(&self) -> String {
        format!("local://{}", self.port)
    }

    fn connect(&self) {
        println!("[Local] Connecting to local port {}", self.port);
    }
}

// ---------------------------------------------------------------------
// Generic and Dynamic use functions
// ---------------------------------------------------------------------

/// 🧩 Static dispatch: Compile-time monomorphization of `Connectable`
/// The compiler generates specialized code for each type of T.
pub fn use_connection_generic<T: Connectable>(conn: &T) {
    println!("→ [generic] {}", conn.describe());
    conn.connect();
    println!();
}

/// 🌀 Dynamic dispatch: Single runtime entry point for Connectable trait objects
pub fn use_connection_dyn(conn: &dyn Connectable) {
    println!("→ [dyn] {}", conn.describe());
    conn.connect();
    println!();
}