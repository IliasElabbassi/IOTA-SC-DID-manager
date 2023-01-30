#!/bin/bash

# import the public key that is used to sign he software reelase
wget -qO - https://ppa.hornet.zone/pubkey.txt | sudo apt-key add -


# add the Hornet repo to the APT sources
sudo sh -c 'echo "deb http://ppa.hornet.zone stable main" >> /etc/apt/sources.list.d/hornet.list'

# Update APT package list and install Hornet
sudo apt update
sudo apt install hornet

# Enable the systemd service
sudo systemctl enable hornet.service

# to find the Hornet config file go to : /var/lib/hornet