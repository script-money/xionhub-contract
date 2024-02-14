# xionhub contract

This is the contract for xionhub, a web3 insight sharing platform, power by [xion](https://xion.burnt.com/), the first L1 blockchain purpose built for consumer adoption.

## how to deploy

1. install local environment following [this](https://docs.burnt.com/xion/develop/get-started-guide/setup/installation-prerequisites-setup-local-environment)
2. `cargo test` make sure all tests pass
3. `RUSTFLAGS='-C link-arg=-s' cargo wasm` build wasm
4. If xion account created in docker, use `xiond keys list --keyring-backend test --home ~/OrbStack/docker/volumes/xion_testnet-data/chain-data/` check the key name， --home is the path of xiond data in volume
5. set $TXFLAG, `export XION_TXFLAG=(--node "https://rpc.xion-testnet-1.burnt.com:443/" --chain-id "xion-testnet-1" --gas-prices "0uxion" --gas "auto" --gas-adjustment "1.4" --keyring-backend "test" --home "~/OrbStack/docker/volumes/xion_testnet-data/chain-data/")`
6. store wasm `RES=$(xiond tx wasm store target/wasm32-unknown-unknown/release/xionhub_contract.wasm $XION_TXFLAG --from account1)`, account1 is key name.
7. `echo $RES` will show TX_HASH, then go to `https://explorer.burnt.com/xion-testnet-1/tx/[TX_HASH]` search "code_id" in page
8. initiate contract with `xiond tx wasm instantiate "165" "{}" --from account1 --label "xionhub testnet" $XION_TXFLAG --no-admin`, 165 is code_id, with empty init_msg and no-admin
9. get contract address with `xiond query wasm list-contract-by-code 164 --node "https://rpc.xion-testnet-1.burnt.com:443/" --output json | jq -r '.contracts[0]'`
