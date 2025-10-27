mod connection_builder_a;
mod connection_builder_b;

// fn main() {
//     let conn = crate::connection_builder_a::ConnectionBuilder::default()
//         .address("10.0.0.1")
//         .protocol(crate::connection_builder_a::Protocol::Tcp)
//         .port(8080)
//         .build()
//         .expect("Unable to create connection");
//
//
//     println!("{conn:?}");
// }



// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut connections = Vec::new();
//
//     // ❌ For every connection, we re-build the entire builder — repetitive, error prone
//     for port in 8080..8180 {
//         let conn = crate::connection_builder_a::ConnectionBuilder::default()
//             .address("10.0.0.1") // repeated every loop
//             .protocol(crate::connection_builder_a::Protocol::Tcp) // repeated every loop
//             .port(port)              // only thing that really changes
//             .build()?;
//
//         connections.push(conn);
//     }
//
//     println!("Built {} connections.", connections.len());
//     println!("First: {:#?}", connections.first());
//     println!("Last:  {:#?}", connections.last());
//
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Define immutable attributes once
//     let base_builder = crate::connection_builder_b::ConnectionBuilder::default()
//         .address("10.0.0.1")
//         .protocol(crate::connection_builder_b::Protocol::Tcp);
//
//     // Generate 100 different connections by only varying port
//     let connections: Vec<crate::connection_builder_b::Connection> = (8080..8180)
//         .map(|p| base_builder.port(p).build().unwrap())
//         .collect();
//
//     // Show the first and last to confirm
//     println!("{:?}", connections.first());
//     println!("{:?}", connections.last());
//
//     Ok(())
// }


// ------------------------------------------------------------
// Demo: build connections for 3 different address groups
// ------------------------------------------------------------
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // one shared default builder with base configuration
    let base_builder = crate::connection_builder_b::ConnectionBuilder::default().protocol(crate::connection_builder_b::Protocol::Tcp);

    // define our group configs
    let groups = vec![
        ("10.0.0.1", 8080..8090),
        ("10.0.0.2", 8090..8100),
        ("127.0.0.1", 8100..8110),
    ];

    let mut connections = Vec::new();

    for (addr, ports) in groups {
        // create an address-specific builder layer
        let addr_builder = base_builder.address(addr);

        for port in ports {
            let conn = addr_builder.port(port).build()?;
            connections.push(conn);
        }
    }

    println!("Built {} connections:", connections.len());
    for (i, c) in connections.iter().enumerate() {
        println!("{:>2}: {:?}:{:?}", i, c.address, c.port);
    }

    Ok(())
}