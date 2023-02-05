### IOTA idenity informations

- IOTA indenty is an API we cannot implement it in a Smart contract
- the API allows us to create identity, delete identity and more

- we need to create a rust script to interract with the API
- Create a Cargo environemen ```cargo new did_test```
- update dependencies has follow :

```
[package]
name = "did_test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
identity_iota = "0.6.1"
iota-client = { version = "2.0.1-rc", default-features = false, features = ["tls", "stronghold"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"
```

How to implement ?

- the two IOT devices need to have a DID
- when they want to communicate they need to transfert 


### IOTA indentity rust

https://github.com/iotaledger/identity.rs
https://wiki.iota.org/identity.rs/introduction

#### Decentralized Idenity

https://wiki.iota.org/identity.rs/decentralized_identity/


Decentralized Identity (DID) in IOTA refers to a digital identity that is stored on a decentralized ledger and controlled by the individual who owns the identity. In other words, a DID allows an individual to securely store and manage their personal information and digital assets in a way that is transparent, secure, and under their control.

In the context of IOTA, a DID can be used to represent a person, device, or organization and is stored on the Tangle, the distributed ledger technology that powers IOTA. A DID in IOTA can be used to securely store and manage a wide range of information and assets, such as personal information, credentials, and digital assets.

One of the key benefits of a DID in IOTA is that it provides a secure and transparent way for individuals to control their own digital identity, without relying on centralized entities. This helps to address many of the privacy and security concerns associated with centralized identity management systems.

Overall, decentralized identity is an important aspect of the vision for a more secure and privacy-preserving digital world, and IOTA's implementation of DIDs is a key part of this vision.

#### How does it work ?

A Decentralized Identity (DID) in IOTA works by creating a unique digital identity that is stored on the Tangle, the distributed ledger technology that powers IOTA.

The process of creating a DID in IOTA typically involves the following steps:

- <b>Generation of a private-public key pair</b>: This step involves generating a private key, which is kept secret, and a public key, which is used to identify the DID on the Tangle.

- <b>Creation of a DID document</b>: The DID document is a <b>JSON-LD</b> document that contains information about the DID, such as its public key, authentication methods, and service endpoints. The document is signed with the private key and stored on the Tangle.

- <b>DID Resolution</b>: When a user wants to access their DID, <b>they can use a DID resolver to retrieve the DID document from the Tangle</b>. The resolver uses the DID to look up the associated document and returns it to the user.

- Authentication and Verification: To authenticate and verify the DID, the user can use the public key stored in the DID document. This allows them to prove their identity and access their digital assets.

Overall, the process of creating and using a DID in IOTA is designed to be secure, transparent, and under the control of the individual. By using the Tangle as the underlying technology, IOTA provides a decentralized and trustless environment for DIDs, ensuring that individuals are in control of their digital identity and information.

##### JSON-LD 

JSON-LD (JavaScript Object Notation for Linked Data) is a lightweight, syntax for representing Linked Data in JSON format. 

Here is an example of this format in IOTA DID :

``` JSON
{
  "@context": "https://w3id.org/did/v1",
  "id": "did:iota:abcdefghijklmnopqrstuvwxyz1234567890",
  "publicKey": [{
    "id": "did:iota:abcdefghijklmnopqrstuvwxyz1234567890#keys-1",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:iota:abcdefghijklmnopqrstuvwxyz1234567890",
    "publicKeyBase58": "abcdefghijklmnopqrstuvwxyz1234567890"
  }],
  "authentication": [{
    "type": "Ed25519SignatureAuthentication2018",
    "publicKey": "did:iota:abcdefghijklmnopqrstuvwxyz1234567890#keys-1"
  }],
  "service": [{
    "id": "did:iota:abcdefghijklmnopqrstuvwxyz1234567890;indx",
    "type": "IndxService",
    "serviceEndpoint": "https://indx.iota.org/"
  }]
}
```

#### DID Example

https://wiki.iota.org/identity.rs/tutorials/validate_university_degree/

![Alt text](./img/did-example.png)

Steps :
- Holder: Create a DID
    - ```npm run start create-did alice alice-password```
- Issuer: Create a DID
    - ```npm run start create-did uni-of-oslo uni-password```
- Issuer: Add a Verification Method
    - npm run start create-vm uni-of-oslo uni-password key-1
- Holder: Add a Verification Method
    - npm run start create-vm alice alice-password key-1
- Issuer: Create Revocation list
    - npm run start add-revocation-list uni-of-oslo uni-password rev-1
- Issuer: Create Verifiable Credential
    - npm run start create-vc uni-of-oslo uni-password alice <subjectDid> key-1 rev-1 5
- Holder: Create Verifiable Presentation
    - npm run start create-vp alice alice-password alice-credential.json key-1 xyz123
- Verifier: Verification
    - npm run start verify-vp alice-presentation.json xyz123
- Issuer: Revocation
    - npm run start revoke-vc uni-of-oslo uni-password rev-1 5
