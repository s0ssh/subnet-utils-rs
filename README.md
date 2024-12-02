# subnet-utils

Simple IP subnet utilities


## Usage

To use `subnet-utils`, first add this to your `Cargo.toml`:

```toml
[dependencies]
subnet-utils = "0.1"
```


## Examples

**Check if subnet contains an address.**

```rust
use std::net::{IpAddr, Ipv4Addr};
use subnet_utils::addr_in_subnet;

let res = addr_in_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), "192.168.182.0/24").unwrap();
assert!(res);
```

**Check if any subnet contains an address.**

```rust
use std::net::{IpAddr, Ipv4Addr};
use subnet_utils::addr_in_any_subnet;

let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
let res = addr_in_any_subnet(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
assert!(res);
```

**Check if all subnets contain an address.**

```rust
use std::net::{IpAddr, Ipv4Addr};
use subnet_utils::addr_in_all_subnets;

let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
let res = addr_in_all_subnets(&IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), &subnets).unwrap();
assert!(res);
```

**Check if any subnet contains any address.**

```rust
use std::net::{IpAddr, Ipv4Addr};
use subnet_utils::any_addr_in_any_subnet;

let addrs = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 182, 1)), IpAddr::V4(Ipv4Addr::new(192, 168, 182, 2))];
let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
let res = any_addr_in_any_subnet(&addrs, &subnets).unwrap();
assert!(res);
```


## License

> Copyright &copy; 2024 s0s

This software is released under:

- The MIT License, (http://opensource.org/licenses/MIT)

Please see the license file ([LICENSE.md](LICENSE.md)) for more information.
