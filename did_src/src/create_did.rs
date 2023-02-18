// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::io;

use identity_iota::account::Account;
use identity_iota::account::IdentitySetup;
use identity_iota::account::Result;
use identity_iota::account_storage::Stronghold;
use identity_iota::client::ExplorerUrl;
use identity_iota::iota_core::IotaDID;

#[tokio::main]
pub async fn create() -> Result<()> {
    pretty_env_logger::init();

    let mut user_input_psw = String::new();
    let stdin = io::stdin();
    
    println!("insert password for did creation : ");
    stdin.read_line(&mut user_input_psw);
    println!("creating your new DID document please wait...\n");

    // Sets the location and password for the Stronghold
    //
    // Stronghold is an encrypted file that manages private keys.
    // It implements best practices for security and is the recommended way of handling private keys.
    let stronghold_path: PathBuf = "./did-strong.hodl".into();
    let password: String = user_input_psw.to_owned();
    let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, None).await?;
    
    // Create a new identity using Stronghold as local storage.
    //
    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let account: Account = Account::builder()
        .storage(stronghold)
        .create_identity(IdentitySetup::default())
        .await?;

    // Retrieve the did of the newly created identity.
    let iota_did: &IotaDID = account.did();

    // Print the local state of the DID Document
    //println!("[DID Creation] Local Document from {} = {:#?} \n", iota_did, account.document());
    println!("{}", iota_did);
    // Prints the Identity Resolver Explorer URL.
    // The entire history can be observed on this page by clicking "Loading History".
    // let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
    // println!(
    //     "[DID Creation] Explore the DID Document = {} \n",
    //     explorer.resolver_url(iota_did)?
    // );

    Ok(())
}