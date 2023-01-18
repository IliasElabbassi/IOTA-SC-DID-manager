#!/bin/bash

# use the systemd service to start running Hornet on the Mainnet
if [ $1 = 'start' ]; then
    echo "starting Hornet note connected to the main tangle ..."
    sudo service hornet start
    echo "Node started !"
    exit 0
fi

# You can display the nodes logs
if [ $1 = 'display' ]; then
    echo "Displaying the nodes logs..."
    journalctl -fu hornet
    exit 0
fi

# restart hornet by running
if [ $1 = 'restart' ]; then
    echo "Restarting Hornet node..."
    sudo systemctl restart hornet
    echo "Node restarted!"
    exit 0
fi

# stop hornet
if [ $1 = 'stop' ]; then
    echo "Stoping Hornet node..."
    sudo systemctl stop hornet
    echo "Node stoped!"
    exit 0
fi
