mod connection_enum;
mod connection_builder_a;
mod connection_builder_b;
mod connection_traits;
mod connection_hybrid;

///////////////////////////////
//  Review 1) usage 2) implementation
///////////////////////////////

// fn main() {
//     let connections = vec![
//         connection_enum::Connection::Tcp {
//             address: "10.0.0.1".to_string(),
//             port: 443,
//             encryption: true,
//         },
//         connection_enum::Connection::Udp {
//             address: "10.0.0.2".to_string(),
//             port: 8080,
//         },
//         connection_enum::Connection::LocalHost {
//             port: 9000
//         },
//     ];
// 
//     for conn in &connections {
//         connection_enum::use_connection(conn);
//     }
// }


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let conn = connection_builder_a::ConnectionBuilder::default()
//         .address("10.0.0.1")
//         .protocol(crate::connection_builder_a::Protocol::Tcp)
//         .port(8080)
//         .build()?;
// 
//     connection_builder_a::use_connection(&conn);
// 
//     // NOTE: another possible example
//     // let conn = maybe_port
//     // .map(|p| builder.port(p))
//     // .unwrap_or(builder.port(8080))
//     // .build()?;
// 
//     Ok(())
// }

// 
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
//         connections.push(conn); //single simple arg
//     }
// 
//     println!("Built {} connections.", connections.len());
//     connection_builder_a::use_connection(connections.first().expect("no connections"));
//     connection_builder_a::use_connection(connections.last().expect("no connections"));
//     Ok(())
// }


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Define immutable attributes once
//     let base_builder = connection_builder_b::ConnectionBuilder::default()
//         .address("10.0.0.1")
//         .protocol(crate::connection_builder_b::Protocol::Tcp);
// 
//     // Generate 100 different connections by only varying port
//     let connections: Vec<crate::connection_builder_b::Connection> = (8080..8180)
//         .map(|p| base_builder.port(p).build().expect("unable to build connection"))
//         .collect();
// 
//     println!("Built {} connections.", connections.len());
//     println!(" First {} ", connection_builder_b::use_connection(connections.first().expect("no connections")));
//     println!(" Last {} ", connection_builder_b::use_connection(connections.last().expect("no connections")));
// 
//     Ok(())
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let groups = vec![
        ("10.0.0.1", 8080..8090),
        ("10.0.0.2", 8090..8100),
        ("127.0.0.1", 8100..8110),
    ];

    let mut connections = Vec::new();

    // one shared default builder with base configuration
    let base_builder = connection_builder_b::ConnectionBuilder::default()
        .protocol(crate::connection_builder_b::Protocol::Tcp);


    let mut addr_specific_builders = Vec::new();

    for (addr, ports) in groups {
        // create an address-specific builder layer
        let addr_builder = base_builder.address(addr);
        //we can hold the builder since it will not be modified again
        addr_specific_builders.push(addr_builder.clone());

        for port in ports {
            connections.push(addr_builder.port(port).build()?);
        }
    }

    //now we add port 11000 to every ip
    addr_specific_builders.iter()
        .flat_map(|builder|builder.port(11000).build())
        .for_each(|c| connections.push(c));

    println!("Built {} connections:", connections.len());
    for (i, c) in connections.iter().enumerate() {
        println!("{:>2}: {:?}", i, connection_builder_b::use_connection(c));
    }

    Ok(())
}


// fn main() {
//     let tcp = connection_traits::TcpConnection {
//         address: "10.0.0.1".to_string(),
//         port: 443,
//         encryption: true,
//     };
//     let udp = connection_traits::UdpConnection {
//         address: "10.0.0.2".to_string(),
//         port: 8080,
//     };
//     let local = connection_traits::LocalHostConnection {
//         port: 9000
//     };
// 
//     println!("--- Static dispatch <T: Connectable> ---");
//     connection_traits::use_connection_generic(&tcp);
//     connection_traits::use_connection_generic(&udp);
//     connection_traits::use_connection_generic(&local);
// 
//     println!("--- Dynamic dispatch dyn Connectable ---");
//     // we can hold heterogeneous types behind trait objects
//     let connections: Vec<Box<dyn connection_traits::Connectable>> = vec![
//         Box::new(tcp),
//         Box::new(udp),
//         Box::new(local),
//     ];
// 
//     for conn in connections.iter() {
//         connection_traits::use_connection_dyn(conn.as_ref());
//     }
// }


// fn main() {
//     let connections = vec![
//         connection_hybrid::Connection::Tcp(Box::new(connection_hybrid::TcpConnection {
//             address: "10.0.0.1".to_string(),
//             port: 443,
//             encryption: true,
//         })),
//         connection_hybrid::Connection::Udp(Box::new(connection_hybrid::UdpConnection {
//             address: "10.0.0.2".to_string(),
//             port: 8080,
//         })),
//         connection_hybrid::Connection::Local(Box::new(connection_hybrid::LocalHostConnection { port: 9000 })),
//     ];
// 
//     println!("--- Using Enum Interface ---");
//     for conn in &connections {
//         connection_hybrid::use_enum_connection(conn);
//     }
// 
//     println!("--- Using Trait Interface via Enum ---");
//     for conn in &connections {
//         connection_hybrid::use_connection_trait(conn.as_dyn());
//     }
// }

