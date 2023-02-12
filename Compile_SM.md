#### Comment compiler un Smart Contract IOTA

#### Prérequis

- Rust
- Wasm Pack
- Cargo

#### Etapes

- Crée un environement Cargo avec la commande ```cargo new <project_name>``` ou bien grace a la commande `schema -init yourEnv`

voici comment le folder creer par Cargo est constituer
```
* Cargo.toml        # define dependencies of the smart contract
* src/lib.rs        # Source of the smart contract
* pkg/              # location du fichier wasm
* target/
```

- Compile le smart contract et generer le fichier wasm ```wasm-pack build```

#### Rust dependencies

- wasmlib : essential for smart contract
- console_error_panic_hook : better debugging of panics
- wee_alloc : tiny allocator for wasm
- chrono : for date and time handling/manipulation
- serde : for serialization / deserialization