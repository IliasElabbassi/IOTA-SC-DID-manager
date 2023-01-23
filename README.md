# PFE

### Hornet

- <b>Dashboard</b> : http://localhost:8081/
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

<u>solution</u> :
https://stackoverflow.com/questions/52604068/using-wsl-ubuntu-app-system-has-not-been-booted-with-system-as-init-system-pi
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

### Useful Information

- Smart contracts run on Wasp nodes on layer 2 in connection with Goshimmer on layer 1, responsible for conducting transactions and messaging (see next figure).
![GoShimmerWasp](./img/GoShimmerWasp.png "GoShimmerWasp")
Due to the separation of Wasp and Goshimmer, they need a technical capability to connect and interact with each other. Therefore, Goshimmer nodes contain a “txstream” plugin, which needs to be activated for connecting Wasp nodes.
<br>
- To test smart contracts and run unit tests against smart contract functionality, the “solo” environment written in Go can be used.
<br>
-  The network hosts a layer 1 Goshimmer node for messaging and a layer 2 Wasp node for running smart contracts. 
<br>
- A Wasp Node has it's own wallet. Where we need to send funds to, to deploy smart contracts.
<br>
- Smart contracts for the IOTA network can be implemented in Rust and then compiled to a WebAssembly file.
<br>
- Smart contract building env
```
* Cargo.toml        # define dependencies of the smart contract
* src/lib.rs        # Source of the smart contract
* pkg/      
* target/
```

### Questions

- What is the main stack for developing rust smart contracts on IOTA? I know that I need to have a wasp node, where the smart contract will be deployed from. But from where is it deployed ? Do I need to run an Hornet node or Goshimmer node ?
<br>
- Whats the difference between Hornet and Goshimmer and Wasp ? And what are their purpose and differences ?
<br>


### Memo

- use Docker on WSL2 : 
    - Download Docker Desktop for Windows.
    - From the Docker menu, select Settings and then General.
    - Select the Use WSL 2 based engine check box. 
    - Select Apply & Restart.