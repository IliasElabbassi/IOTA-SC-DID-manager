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

### Main.rs

We can create a DID document by inserting a password into the terminal : 

```cd [cargo root project]/target/debug] ```
``` ./did_src ```
``` $ insert password for did creation : mypassword1234 ```

out :
```
[DID Creation] Local Document from did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT = IotaDocument {
    document: CoreDocument {
        id: did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT,
        controller: None,
        also_known_as: {},
        verification_method: {},
        authentication: {},
        assertion_method: {},
        key_agreement: {},
        capability_delegation: {},
        capability_invocation: {
            VerificationMethod {
                id: did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT#sign-0,
                controller: did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT,
                type_: Ed25519VerificationKey2018,
                data: PublicKeyMultibase(zFfBeF5BEYt6bRYKgpyTu1DtejJinEKYexEB6PmrY5tCz),
                properties: {},
            },
        },
        service: {},
        properties: {},
    },
    metadata: IotaDocumentMetadata {
        created: Some(
            "2023-02-05T22:57:13Z",
        ),
        updated: Some(
            "2023-02-05T22:57:13Z",
        ),
        previous_message_id: MessageId(0000000000000000000000000000000000000000000000000000000000000000),
        properties: {},
    },
    proof: Some(
        Proof {
            type_: "JcsEd25519Signature2020",
            value: Signature(2rJkUwESMf2TdxkjLFMLJZcusAa3kG4zRQSjXzrSuqcypzRERnUZuCh95wvSKijPwSqt9eZnhiBYu3e69PdiZPe5),
            method: "did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT#sign-0",
            created: None,
            expires: None,
            challenge: None,
            domain: None,
            purpose: None,
        },
    ),
}
```
```
[DID Creation] Explore the DID Document = https://explorer.iota.org/mainnet/identity-resolver/did:iota:GTrSAuDVyqSGYk5H2HZQFbcduNX9MjJSrJVqfCMyMZKT
```

Your did strongold is located on the ```[cargo root project]/did-strong.hodl```