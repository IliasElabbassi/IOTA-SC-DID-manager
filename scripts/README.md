### Info about Script Folder

Inside this folder you will find 3 scripts :
- `DID_manager.sh`
- `Setup_predefined_dev_env.sh`
- `Wasp_install.sh`

Each file have it's own purpose let me explain them briefly :
- `Wasp_install.sh` : in this file a little record of the command to use to install the IOTA environement on your machine.
- `Setup_predefined_dev_env.sh` : this file describe you how to launch an environnement clicky thanks to the docker files located on the wasp repository
- `DID_manager.sh`: this file is the main file for interracting withe DID IOTA Smart Contract (ISC) CRUD. It implement
    - Adding a DID
    - Updating a DID
    - Deleting a DID
    - Viewing a specific DID
    - Viewing all the dids:

#### Important information the DID Manager

Before using it be sure to init the wasp-cli config file inside this folder.
- you should put the good corresponding endpoint to the hornet layer and wasp layer.
- Also you should put your own seed.