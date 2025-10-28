// ------------------------------------------------------------
// connection_hybrid.rs
// ------------------------------------------------------------

use std::fmt::Debug;

// ---------------------------------------------------------------------
// 1️⃣ Shared behavior: the Trait
// ---------------------------------------------------------------------

/// The `Connectable` trait represents behavior shared across all connection types.
/// Anything that can `describe()` itself and `connect()` can implement this trait.
pub trait Connectable: Debug {
    fn describe(&self) -> String;
    fn connect(&self);
}

// ---------------------------------------------------------------------
// 2️⃣ Concrete types implementing the trait
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
// 3️⃣ Enum wraps our different connection types
// ---------------------------------------------------------------------
//
// This is the *hybrid pattern*:
// - The enum acts as a closed set of known categories (TCP, UDP, LOCAL).
// - Each variant holds a `Box<dyn Connectable>` trait object, which allows
//   behavior polymorphism inside that variant.
//
// Said another way: The enum gives us data *tags*,
// and the trait objects give us dynamic *behavior*.
// ---------------------------------------------------------------------

#[derive(Debug)]
pub enum Connection {
    Tcp(Box<dyn Connectable>),
    Udp(Box<dyn Connectable>),
    Local(Box<dyn Connectable>),
}

impl Connection {
    /// Return a string describing the connection (delegates to trait implementation)
    pub fn describe(&self) -> String {
        match self {
            Self::Tcp(c) => c.describe(),
            Self::Udp(c) => c.describe(),
            Self::Local(c) => c.describe(),
        }
    }

    /// Execute the connect action for whichever variant we hold
    pub fn connect(&self) {
        match self {
            Self::Tcp(c) => c.connect(),
            Self::Udp(c) => c.connect(),
            Self::Local(c) => c.connect(),
        }
    }

    /// Helper: provides a `&dyn Connectable` reference for unified API use.
    ///
    /// `Box` implements `as_ref()` to return `&dyn Trait` cleanly,
    /// avoiding explicit deref chains like `&**c`.
    /// This makes the logic clearer for teaching and presentation.
    pub fn as_dyn(&self) -> &dyn Connectable {
        match self {
            Self::Tcp(c) => c.as_ref(),
            Self::Udp(c) => c.as_ref(),
            Self::Local(c) => c.as_ref(),
        }
    }
}

// ---------------------------------------------------------------------
// 4️⃣ Demo helpers showing both Enum and Trait views
// ---------------------------------------------------------------------

/// Demonstrates direct use of the enum interface (pattern matching)
pub fn use_enum_connection(conn: &Connection) {
    println!("→ [enum] {}", conn.describe());
    conn.connect();
    match conn {
        Connection::Tcp(_) => println!("   ↳ handled as TCP variant\n"),
        Connection::Udp(_) => println!("   ↳ handled as UDP variant\n"),
        Connection::Local(_) => println!("   ↳ handled as Local variant\n"),
    }
}

/// Demonstrates use through a trait object reference (`&dyn Connectable`)
pub fn use_connection_trait(conn: &dyn Connectable) {
    println!("→ [trait] {}", conn.describe());
    conn.connect();
    println!();
}