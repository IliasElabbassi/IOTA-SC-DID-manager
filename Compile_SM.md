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

#### Schema tool

- We need to have the wasp repo cloned localy.
- After that add the bin folder inside the Wasp repo to the Path `export PATH=$PATH:$(go env GOPATH)/bin` assuming you already have Go
- we then need to create a Schema space with `schema -init nameOfyourEnv`
- then go inside this folder
- You will see a `schema.yaml` file, this is the core of out Smart contract, by specifying each state, params and functions into this file it will generate all you core Smart contract.
- to generate the code use `schema -rs` for rust code or `schema -go` for go.
- then move inside the `nameOfyourEnvimpl` folder created, you will see the `funcs.rs` file, it's the only file you will need to change to create the logic of your contract.
- to build go inside the `nameOfyourEnvwasm` folder created, and run `wasm-pack build`
- You now have the wasm file located on the `pkg` folder

#### Rust dependencies

- wasmlib : essential for smart contract
- console_error_panic_hook : better debugging of panics
- wee_alloc : tiny allocator for wasm
- chrono : for date and time handling/manipulation
- serde : for serialization / deserialization