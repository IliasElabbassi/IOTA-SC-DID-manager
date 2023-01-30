### IOTA idenity informations

- IOTA indenty is an API we cannot implement it in a Smart contract
- the API allows us to create identity, delete identity and more

- we need to create a rust script to interract with the API
- Create a Cargo environemen ```cargo new did_test```
- update dependencies has follow :

```
[package]
name = "did_test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
identity_iota = "0.6.1"
iota-client = { version = "2.0.1-rc", default-features = false, features = ["tls", "stronghold"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"
```