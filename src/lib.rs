//! # subnet-utils
//!
//! Simple IP subnet utilities
//!
//! ## Usage
//! 
//! To use `subnet-utils`, first add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! subnet-utils = "0.1"
//! ```
//! 
//! ## Examples
//!
//! ### Check if subnet contains an address.
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr};
//! use subnet_utils::addr_in_subnet;
//!
//! let res = addr_in_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), "192.168.182.0/24").unwrap();
//! assert!(res);
//! ```
//!
//! ### Check if any subnet contains an address.
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr};
//! use subnet_utils::addr_in_any_subnet;
//!
//! let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
//! let res = addr_in_any_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
//! assert!(res);
//! ```
//!
//! ### Check if all subnets contain an address.
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr};
//! use subnet_utils::addr_in_all_subnets;
//!
//! let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
//! let res = addr_in_all_subnets(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
//! assert!(res);
//! ```
//!
//! ### Check if any subnet contains any address.
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr};
//! use subnet_utils::any_addr_in_any_subnet;
//!
//! let addrs = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), IpAddr::V4(Ipv4Addr::new(192, 168, 182, 2))];
//! let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
//! let res = any_addr_in_any_subnet(&addrs, &subnets).unwrap();
//! assert!(res);
//! ```


use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use netaddr2::{Contains, Error as NetError, NetAddr};


/// # Examples
///
/// ### Check if subnet contains an address.
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr};
/// use subnet_utils::addr_in_subnet;
///
/// let res = addr_in_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), "192.168.182.0/24").unwrap();
/// assert!(res);
/// ```
pub fn addr_in_subnet(addr: &IpAddr, subnet: &str) -> Result<bool, Box<dyn Error>> {
    match subnet.parse::<NetAddr>() {
        Ok(NetAddr::V4(subnet4)) => {
            Ok(addr.is_ipv4() && subnet4.contains(addr.into()))
        }
        Ok(NetAddr::V6(subnet6)) => {
            Ok(addr.is_ipv6() && subnet6.contains(addr.into()))
        }
        Err(NetError::ParseError(e)) => Err(e.into()),
    }
}


/// # Examples
///
/// ### Check if any subnet contains an address.
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr};
/// use subnet_utils::addr_in_any_subnet;
///
/// let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
/// let res = addr_in_any_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
/// assert!(res);
/// ```
pub fn addr_in_any_subnet(addr: &IpAddr, subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    for subnet in subnets.iter() {
        match subnet.parse::<NetAddr>() {
            Ok(NetAddr::V4(subnet4)) => {
                if addr.is_ipv4() && subnet4.contains(addr.into()) {
                    return Ok(true)
                }
            }
            Ok(NetAddr::V6(subnet6)) => {
                if addr.is_ipv6() && subnet6.contains(addr.into()) {
                    return Ok(true)
                }
            }
            Err(NetError::ParseError(e)) => return Err(e.into()),
        }
    }
    Ok(false)
}

/// # Examples
/// 
/// ### Check if all subnets contain an address.
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr};
/// use subnet_utils::addr_in_all_subnets;
///
/// let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
/// let res = addr_in_all_subnets(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
/// assert!(res);
/// ```
pub fn addr_in_all_subnets(addr: &IpAddr, subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    for subnet in subnets.iter() {
        if let Ok(false) = addr_in_subnet(addr, &subnet) {
            return Ok(false);
        }
    }
    Ok(true)
}

/// # Examples
///
/// ### Check if any subnet contains any address.
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr};
/// use subnet_utils::any_addr_in_any_subnet;
///
/// let addrs = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), IpAddr::V4(Ipv4Addr::new(192, 168, 182, 2))];
/// let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
/// let res = any_addr_in_any_subnet(&addrs, &subnets).unwrap();
/// assert!(res);
/// ```
pub fn any_addr_in_any_subnet(addrs: &Vec<IpAddr>, subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    for subnet in subnets.iter() {
        match subnet.parse::<NetAddr>() {
            Ok(NetAddr::V4(subnet4)) => {
                for addr in addrs.iter() {
                    if addr.is_ipv4() && subnet4.contains(addr.into()) {
                        return Ok(true)
                    }
                }
            }
            Ok(NetAddr::V6(subnet6)) => {
                for addr in addrs.iter() {
                    if addr.is_ipv6() && subnet6.contains(addr.into()) {
                        return Ok(true)
                    }
                }
            }
            Err(NetError::ParseError(e)) => return Err(e.into()),
        }
    }
    Ok(false)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr_in_subnet() {
        let res = addr_in_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), "192.168.182.0/24").unwrap();
        assert!(res);
    }

    #[test]
    fn test_addr_not_in_subnet() {
        let res = addr_in_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 183, 1)), "192.168.182.0/24").unwrap();
        assert!(!res);
    }

    #[test]
    fn test_addr_in_any_subnet() {
        let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
        let res = addr_in_any_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_addr_not_in_any_subnet() {
        let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
        let res = addr_in_any_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 183, 1)), &subnets).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_addr_in_all_subnets() {
        let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
        let res = addr_in_all_subnets(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_addr_not_in_all_subnets() {
        let subnets = vec!["192.168.182.0/24", "192.168.182.2/32"];
        let res = addr_in_all_subnets(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_any_addr_in_any_subnet() {
        let addrs = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), IpAddr::V4(Ipv4Addr::new(192, 168, 182, 2))];
        let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
        let res = any_addr_in_any_subnet(&addrs, &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_any_addr_not_in_any_subnet() {
        let addrs = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), IpAddr::V4(Ipv4Addr::new(192, 168, 182, 2))];
        let subnets = vec!["192.168.181.0/24", "192.168.182.3/32"];
        let res = any_addr_in_any_subnet(&addrs, &subnets).unwrap();
        assert!(!res);
    }
}

