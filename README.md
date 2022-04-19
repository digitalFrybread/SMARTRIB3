<div align="center">

  <h1><code>SMARTRIB3-NDN</code></h1>

  <strong>Next generation blockchain being developed for tribal communities using <a href="https://github.com/paritytech/substrate">Substrate</a> framwork.</strong>

  <h3>
    <a href="https://substrate.io/">Docs</a>
    <span> | </span>
    <a href="https://matrix.to/#/!HzySYSaIhtyWrwiwEV:matrix.org?via=matrix.parity.io&via=matrix.org&via=web3.foundation">Chat</a>
  </h3>

</div>

Proof of stake Testnet based on the [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template).

Featuring custom pallets

* Consensus related pallets: Babe & GRANDPA
* Staking related pallets: staking, session, authorship, im-online, offences, utility
* Governance related pallets: collective, membership, elections-phragmen, democracy, treasure

**Notes:** The code is un-audited and not production ready, use it at your own risk.

## Getting Started

Follow the steps below to get started.

Install rustup:

First, complete the [Dev Docs Installation](https://docs.substrate.io/v3/getting-started/installation/).

Compile:

Use the following command to build the node and run it after build successfully:

```sh
cargo build --release
./target/release/substrate-ndn --dev
```

If you are intrested running a node on Native Derivatives Test Network you can contact <link href="smartrib3@digitalfrybead.com"></link>



 Build spec, `./target/release/substrate-ndn build-spec --chain staging > ndn-staging.json`
 Change original spec to encoded raw spec, `./target/release/substrate-ndn build-spec --chain=ndn-staging.json --raw > ndn-staging-raw.json`
 Start your bootnodes, node key can be generate with command `./target/release/substrate-ndn key generate-node-key`.

 
  ```shell
  ./target/release/substrate-ndn \
       --node-key <your-node-key> \
       --base-path /tmp/bootnode1 \
       --chain ndn-staging-raw.json \
       --name bootnode1
  ```
* Start your initial validators,
  ```shell
  ./target/release/substrate-ndn \
      --base-path  /tmp/validator1 \
      --chain   ndn-staging-raw.json \
      --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
	    --port 30336 \
	    --ws-port 9947 \
	    --rpc-port 9936 \
      --name  validator1 \
      --validator
  ```
