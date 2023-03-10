# PFE

see others md file :
- [IOTA Smart Contract (ISC) Deploy](Deploy_ISC.md)
- [IOTA Smart Contract (ISC) Compile](Compile_SM.md)
- [IOTA Smart Contract (ISC) Interaction](ISC_interaction.md)
- [little DID doc (to review)](DID_doc.md)

### Prerequisites

- Wasp (commit 3cd522ede8b496c0acc34221d56e2bfc76effe39 (HEAD -> develop, origin/develop, origin/HEAD))
- Go 1.19.5
- Linux/Wsl
- Docker
- Rust

### resources

- https://blog.iota.org/iota-smart-contracts-protocol-alpha-release/ IOTA Smart contract explained
- https://docs.rs/identity_iota/latest/identity_iota/  IOTA DID rust doc
- https://wiki.iota.org/shimmer/smart-contracts/guide/core_concepts/smart-contract-anatomy/ anatony of a smart contract in IOTA

### Preconfigured Development Docker

##### Starting
Run ```docker-compose up``` to start the setup.

##### Stopping
Press ```Ctrl-C``` to shut down the setup, but don't press it twice to force it. Otherwise, you can corrupt the Hornet database.

You can also shut down the setup with ```docker-compose down``` in a new terminal.

##### Reset
Run ```docker-compose down --volumes``` to shut down the nodes and to remove all databases.

##### Recreation
If you made changes to the Wasp code and want to use it inside the setup, you need to recreate the Wasp image.

Run ```docker-compose build```

### Seed

Run ```wasp-cli init``` to generate a seed, and you are ready to go.

seed generated : 0x8a79b784060d66b51515ad43d45eefd76034d17c2dce39f1a71a9155c48dcdcb do not use this in mainnet

##### Ports

The nodes will then be reachable under these ports:

- Wasp:
  - API: <http://localhost:9090>
  - Dashboard: <http://localhost:7000> (username: wasp, password: wasp)
  - Nanomsg: tcp://localhost:5550

- Hornet:
  - API: <http://localhost:14265>
  - Faucet: <http://localhost:8091>
  - Dashboard: <http://localhost:8081> (username: admin, password: admin)

### Useful Information

- To test smart contracts and run unit tests against smart contract functionality, the “solo” environment written in Go can be used.

<br>

- Goshimer not used anymore for rust smart contract

<br>

- A Wasp Node has it's own wallet. Where we need to send funds to, to deploy smart contracts.

<br>

- Smart contracts for the IOTA network can be implemented in Rust and then compiled to a WebAssembly file.

<br>

- Rust building env ```cargo new <project_name>```
```
* Cargo.toml        # define dependencies of the smart contract
* src/lib.rs        # Source of the smart contract
* pkg/      
* target/
```
<br>

- compile a rust lib to wasm :
  - Create a new Rust project using cargo
  - add the IOTA contract Library to the `Cargo.toml`file
  - Compile the contract using the  `cargo build --target wasm32-unknown-unknown ` command.
  - Deploy the compiled contract to the IOTA Tangle by sending a transaction containing the WASM bytecode.
<br>

- To deploy a ISC with `wasp-cli` you need to be in the same file than the `wasp-cli.json` config file.

<br>

- wasp-cli is a command line tool for interacting with Wasp and its smart contracts.

<br>

- To generate ISC use the schema tool, it is a tool that make use of the yaml format to generate rust contract. Check this link for more information : https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/schema/

### IOTA Smart Contract (ISC) env building

- We need to have the wasp repo cloned localy.
- After that add the bin folder inside the Wasp repo to the Path `export PATH=$PATH:$(go env GOPATH)/bin` assuming you already have Go
- After that inside the wasp repo use `make wasm` to install schema tool
- we then need to create a Schema space with `schema -init nameOfyourEnv`
- then go inside this folder
- You will see a `schema.yaml` file, this is the core of out Smart contract, by specifying each state, params and functions into this file it will generate all you core Smart contract.
- to generate the code use `schema -rs` for rust code or `schema -go` for go.
- then move inside the `nameOfyourEnvimpl` folder created, you will see the `funcs.rs` file, it's the only file you will need to change to create the logic of your contract.
- to build go inside the `nameOfyourEnvwasm` folder created, and run `wasm-pack build`
- You now have the wasm file located on the `pkg` folder

### Memo

- use Docker on WSL2 : 
    - Download Docker Desktop for Windows.
    - From the Docker menu, select Settings and then General.
    - Select the Use WSL 2 based engine check box. 
    - Select Apply & Restart.

- Clean cargo build `cargo clean`

- add a lib to Cargo.toml via cmd line ex : `cargo add serde_with`

- unregister a wsl distrib : `wsl --unregister <distrib_name>`
  - to get the distrib name use : `wsl -l -v`

- to get feedback after deployting use `wasp-cli chain request <hash given after deployement>`
  
### Deploy

https://wiki.iota.org/shimmer/smart-contracts/guide/chains_and_nodes/setting-up-a-chain/#deploy-the-isc-chain

#### ISC

https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/schema/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/typedefs/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/state/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/params/

### Errors encoutered

<b>System has not been booted with systemd as init system (PID 1). Can't operate.</b>

<u>solution</u> : https://stackoverflow.com/questions/52604068/using-wsl-ubuntu-app-system-has-not-been-booted-with-system-as-init-system-pi
```
sudo nano /etc/wsl.conf
```

```
[boot]
systemd=true
```

```
wsl.exe --shutdown
```

<b>error[E0463]: can't find crate for `core`</b>

<u>solution</u> : https://stackoverflow.com/questions/67898431/errore0463-cant-find-crate-for-core-while-building-rust-project-for-wasm32

```
rustup target add wasm32-unknown-unknown
```

<b>error: failed to run custom build command for `libsodium-sys v0.2.7`</b>

<u>solution</u> : libsodium-sys cant be generated into wasm.


<b>error: sudo: add-apt-repository: command not found </b>

<u>solution</u> : 

```
sudo apt update
sudo apt install software-properties-common
sudo apt update
```

<b>error : Cannot make install into wasp repo</b>

<u>solution : </u> The problem was that some functionnalities used needed go aboce 1.16, but i had 1.13 installed into /usr/bin/go. But the want i wanted to use was located in /usr/local/go.
I had to delete the /usr/bin one. And added the /usr/local one to the path.

<b>error : cannot use "The version of quic-go you're using can't be built on Go 1.20 yet</b>

<u>solution : </u> deletin go and installing version 1.19 (aka the one specified in the go.mod file)

<b>error : cannot find command schema</b>

<u>solution : </u> Go into the wasp repo and and the bin folder to path with ```export PATH=$PATH:$(go env GOPATH)/bin ```

<b>error : Missing Lifetime Operator when generating wasm with `wasm-pack build`</b>

<u>solution : </u> You just need to add `<'a>` to all the `struct` and `ScFunc` in the `contract.rs` file.