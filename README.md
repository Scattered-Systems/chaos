# Chaos

Welcome to Chaos, a virtualized storage solution with an internal blockchain driving each of the interactions. 
Chaos is designed to replace traditional cloud storage providers, giving user's a hyper-secure and globally accessible
personal cloud created from the devices they currently have lying around the house.


## Features
### Application
* Editor
  * Seamlessly manage any file with our lightweight text-editor
  * Enable users to publish finished work to the proper locations with scaffolded CI/CD pipelines
* Storage
  * Enable users to quickly aggregate their information from other service providers
  * Encourage users to rent out excess storage space (to rent more)
  * Allow any device registered with the application to contribute to the users total storage and computing power

### Containers
* Registries - Manage your own self-hosted container and package registries
* Repositories - Manage your own self-hosted, git-based repositories stored on IPFS

### Controller
* Leverage your choice of storage networks (Arweave, Filecoin, IPFS)
* Self-host a globally accessible, private IPFS network


### Utilities
* ChaoticStorage - A personal, virtual storage network 
  * Enable users to partition the space
  * Automate a number of redundant data tasks
  * Integrate with a user-bound, enterprise grade Intelligence engine allowing users to preform their own data analytics

* FileSync - Quickly aggregate your files from Cloud Service Providers
* SIWE - Quickly sign-in with your Ethereum Name
  * When accessed via Proton, the SSO features will shine allowing users to seamlessly cross digital borders

## Getting Started

    git clone https://github.com/scattered-systems/chaos.git

### _Cargo_

    cargo build --release --package chaos
    cargo run --package chaos --bin chaos