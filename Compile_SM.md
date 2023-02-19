#### How to Compile an IOTA Smart Contract

#### Prerequisites

- Rust
- Wasm Pack
- Cargo
- Go 1.19.5

#### Steps

- Create a Cargo environment with the command ```cargo new <project name>``` or with the command ``schema -init yourEnv` (it's reccomended to use the schema tool)

here is how the folder is constituted after using `schema -rs`
```
* rs/
*   my_project_contract/        # location of the backbone of your ISC (dont touch this code)
*   my_project_contractimpl/    # locationf of the funcs.rs file that represent the real logic of your ISC
*   my_project_contractwasm/    # location of the main func of your ISC (dont touch this code)
* test/
* schema.yaml 
```

```
* Cargo.toml                    # define dependencies of the smart contract
* src/lib.rs                    # Source code
* pkg/                          # build sources (where the wasm file is also located)
* target/
```

- to compile a Schema tool project go into the `impl` folder created : `cd rs/my_project_contractimpl`

- Compile the smart contract and generate the wasm file with ```wasm-pack build```

- the wasm file is now located on under the `pkg` folder

#### Schema tool install

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