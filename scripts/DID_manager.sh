#!/bin/bash

IFS=' '
ISCname='appenddid5'
stronHoldFile="./did-strong.hodl"

echo "Please choose what to do :"
echo "[1] create did and push it to the ISC"
echo "[2] update a did in the ISC"
echo "[3] delete a did in the ISC"
echo "[4] view dids in the ISC"
echo "[5] get the did at a specific index"

read which

if [ $which -eq 1 ]; then
    echo "Creating a DID :"
    echo "Please enter a password for the DID : "
    read pass
    if test -f "$stronHoldFile"; then
        echo "Removing old did-strong.hodl file"
        rm ./did-strong.hodl
    fi
    echo "Generating DID..."
    res=$( ../did_src/target/debug/did_src $pass )
    read -ra arr <<< $res
    DID=${arr[0]}
    echo "Generated DID : $DID"
    echo "Uploading DID to ISC..."
    wasp-cli chain post-request $ISCname addDid string newDID string "$DID"
fi

if [ $which -eq 2 ]; then
    echo "Updating a DID :"
    echo "Please enter the DID to update : "
    read toUpdate
    echo "Please enter the value of the new DID : "
    read value
    echo "Updating the DID..."
    wasp-cli chain post-request $ISCname updateDid string toUpdateDID string "$toUpdate" string value string "$value" 
fi

if [ $which -eq 3 ]; then
    echo "Deleting a DID :"
    echo "Please enter the DID to delete : "
    read toDelete
    echo "Deleting the DID..."
    wasp-cli chain post-request $ISCname deleteDid string toDeleteDID string "$toDelete"
fi

if [ $which -eq 4 ]; then
    echo "View DIDs :"
    didLen=$( wasp-cli chain call-view $ISCname getLength | wasp-cli decode string length int8 )
    read -ra didLenArr <<< $didLen
    len=${didLenArr[1]}
    echo "Length of the Array : $len"
    didIndex=0

    while [ $didIndex -le $len ]
    do
        echo "Index: $i"
        wasp-cli chain call-view $ISCname getDID string index int8 $didIndex | wasp-cli decode string indexedDID string
        ((didIndex++))
    done
fi

if [ $which -eq 5 ]; then
    echo "View a DID :"
    echo "Please enter the index of the did you want to view :"
    read indexDID
    echo "Reading the DID..."
    wasp-cli chain call-view $ISCname getDID string index int8 $indexDID | wasp-cli decode string indexedDID string
fi