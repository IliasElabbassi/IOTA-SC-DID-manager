# PFE

see others md file :
- [DID ref](DID_ref.md)
- [Project info](Projet_info.md)
- [IOTA Smart Contract (ISC) compile](Compile_SM.md)

### ressources
- https://blog.iota.org/iota-smart-contracts-protocol-alpha-release/ IOTA Smart contract explained
- https://docs.rs/identity_iota/latest/identity_iota/  IOTA DID rust doc
- https://wiki.iota.org/shimmer/smart-contracts/guide/core_concepts/smart-contract-anatomy/ anatony of a smart contract in IOTA


### Hornet

- <b>Dashboard</b> : <http://localhost:8091>
provides useful information regarding the node's health, peering/neighbors, overall network health, and consumed system resources.

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

### Useful Information

- To test smart contracts and run unit tests against smart contract functionality, the “solo” environment written in Go can be used.
<br>
-  The network hosts a layer 1 Goshimmer node for messaging and a layer 2 Wasp node for running smart contracts.
<br>
- Goshimer not used anymore for rust smart contract
<br>
- A Wasp Node has it's own wallet. Where we need to send funds to, to deploy smart contracts.
<br>
- Smart contracts for the IOTA network can be implemented in Rust and then compiled to a WebAssembly file.
<br>

- Smart contract building env ```cargo new <project_name>```
```
* Cargo.toml        # define dependencies of the smart contract
* src/lib.rs        # Source of the smart contract
* pkg/      
* target/
```
- compile a rust smart contract :

  - Create a new Rust project using cargo
  - add the IOTA contract Library to the `Cargo.toml`file
  - Compile the contract using the  `cargo build --target wasm32-unknown-unknown ` command.
  - Deploy the compiled contract to the IOTA Tangle by sending a transaction containing the WASM bytecode.
<br>

- dependencies to add in the Cargo.toml: 
```
[dependencies]
wasmlib = { git = "https://github.com/iotaledger/wasp", rev = "05516ca" }

[lib]
crate-type = ["cdylib", "rlib"]

[dev-dependencies]
wasm-bindgen-test = "0.3.13"
```
<br>

- wasp node provides :
  - A basic wallet functionality
  - Hashing algorithms
  - A web worker to provide proof of work
  - Construction of On/Off Ledger requests
  - Construction of smart contract arguments - and payloads
  - Generation of seeds (including their - private keys and addresses)
  - Serialization of data into binary messages
  - Deserialization of smart contract state
<br>

- wasp-cli is a command line tool for interacting with Wasp and its smart contracts.



### Questions
ENG
- What is the main stack for developing rust smart contracts on IOTA? I know that I need to have a wasp node, where the smart contract will be deployed from. But from where is it deployed ? Do I need to run an Hornet node or Goshimmer node ?
<br>
- Whats the difference between Hornet and Goshimmer and Wasp ? And what are their purpose and differences ?
<br>


FR
- ais-je besoins de Goshimmer pour deployer et utilisé un smart contract ?
<br>
- Quelle est la différence entre Hornet and Goshimmer ?
<br>
- Quelle est la stack principale pour développer des smart contracts rust sur IOTA ? Je sais que j'ai besoin d'un node Wasp, à partir duquel le smart contract sera déployé. Mais à partir de quoi est-il déployé ? Dois-je utiliser un node Hornet ou un node Goshimmer ?
<br>
- Est ce que vous pouvez m'expliquez le DID (decentralized identity) ?
<br>
- Comment build et déployer un smart contract rust ?
<br>
- Dois-je faire du unit testing avec Go ?

### Memo

- use Docker on WSL2 : 
    - Download Docker Desktop for Windows.
    - From the Docker menu, select Settings and then General.
    - Select the Use WSL 2 based engine check box. 
    - Select Apply & Restart.

- Clean cargo build `cargo clean`

- add a lib to Cargo.toml via cmd line ex : `cargo add serde_with`


### Deploy

https://wiki.iota.org/shimmer/smart-contracts/guide/chains_and_nodes/setting-up-a-chain/#deploy-the-isc-chain

#### ISC

https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/schema/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/typedefs/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/state/
https://wiki.iota.org/shimmer/smart-contracts/guide/wasm_vm/params/