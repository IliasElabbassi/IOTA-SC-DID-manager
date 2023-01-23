# PFE

### Hornet

- <b>Dashboard</b> : http://localhost:8081/
provides useful information regarding the node's health, peering/neighbors, overall network health, and consumed system resources.

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
