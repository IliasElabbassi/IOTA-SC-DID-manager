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