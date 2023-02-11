# prerequis
#GO https://go.dev/doc/install
rm -rf /usr/local/go
sudo tar -C /usr/local -xzf ./libs/go1.19.5.linux-amd64.tar.gz
export PATH=$PATH:/usr/local/go/bin
go version

# Installing the Solidity Compiler
sudo apt-get install software-properties-common
sudo add-apt-repository ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc

# clone wasp repo
git clone https://github.com/iotaledger/wasp
git checkout develop
make install
export PATH=$PATH:$(go env GOPATH)/bin
source ~/.profile

# Rust for wsl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# /home/ilias/.rustup
# /home/ilias/.cargo

# build rust contract
wasm-pack build [contract pwd]

# deploy .wasm contract
wasp-cli chain deploy 