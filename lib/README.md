# avina-lib
API bindings written in Rust for LRZ-specific features of
the Openstack-based LRZ Compute Cloud, [https://cc.lrz.de](https://cc.lrz.de), first and
foremost the budgeting system.

## Usage
To use the library add the following to your `Cargo.toml` under `[dependencies]`:
```toml
avina = 2
```
After that you create a `Token` and `Api` object to interact with the API:
```rust
use avina::{Token, Api};

// let token = Token::from_str("abcdefg...").unwrap();
let token = Token::new(
                auth_url.as_str(),
                username.as_str(),
                password.as_str(),
                project_name.as_str(),
                user_domain_name.as_str(),
                project_domain_id.as_str(),
            ).unwrap();
let api = Api::new("https://cc.lrz.de:1337/api", token, None, None).unwrap();
println!("{:?}", api.user.me());
```
