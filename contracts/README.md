**[SUBSTRATE NODE]**

* alias "contract=../../Developer/substrate-contracts-node/target/release/substrate-contracts-node"
* contract --dev

**[DEPLOY]**

* cargo contract build
* cargo contract upload --suri //Alice
* cargo contract instantiate --suri //Alice --args true

**[CLIENT]**

* cargo install subxt-cli
* subxt metadata --url ws://127.0.0.1:9944 --output-file metadata.scale

