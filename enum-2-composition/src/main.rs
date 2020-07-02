#![allow(dead_code, unused_variables, unused_assignments)]

// Composition using Enum

struct IpV4Addr {
    addr: (u8, u8, u8, u8)
    // "addr" is a tuple
    // Elements are accessed with addr.0, addr.1, addr.2, etc.,
}

struct IpV6Addr {
    addr: (u16, u16, u16, u16, u16, u16, u16, u16)
    // "addr" is a tuple
    // Elements are accessed with addr.0, addr.1, addr.2, etc.,
}

enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr)
}

// Different from
// --------------
//
// struct IpAddr {
//      v4: IpV4Addr,
//      v6: IpV6Addr
// }
//

impl IpAddr {
    // Usage
    // -----
    // let x = IpAddr::new("10.20.1.153");
    //
    fn new(addr: &str) -> IpAddr {
        if addr.contains('.') {
            let ip = IpV4Addr { addr: (0, 0, 0, 0) };
            // Parse the IpV4 string
            // Store the value in "ip"
            return IpAddr::V4(ip);
        }
        else {
            let ip = IpV6Addr { addr: (0, 0, 0, 0, 0, 0, 0, 0) };
            // Parse the IpV6 string
            // Store the value in "ip"
            return IpAddr::V6(ip);
        }
    }

    // Usage
    // -----
    // let x = IpAddr::new("10.20.1.153");
    // println!("x - {}", x.to_string());
    //
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(ip) => {
                /* "ip" refers to the value that is stored in IpAddr::V4 */
                
                let mut s = String::new();
                s = format!("{}.{}.{}.{}", ip.addr.0, ip.addr.1, ip.addr.2, ip.addr.3);
                return s;
            },
            IpAddr::V6(ip) => {
                /* "ip" refers to the value that is stored in IpAddr::V6 */
                
                let mut s = String::new();
                s = format!("{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
                            ip.addr.0, ip.addr.1, ip.addr.2, ip.addr.3, ip.addr.4, ip.addr.5, ip.addr.6, ip.addr.7);
                return s;
            }
        }
    } 
}

fn main() {
    let x: IpAddr = IpAddr::new("10.20.1.153");
    let y: IpAddr = IpAddr::new("00:00:00:00:FF:BE:BA:BC");

    println!("\n x - {}", x.to_string());
    println!("\n y - {}", y.to_string());
}