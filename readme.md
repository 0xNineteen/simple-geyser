- src/lib.rs: entrypoint 
- src/plugin.rs: main plugin code
- to run: `cargo build && solana-test-validator -r --geyser-plugin-config config.json`
- note: make sure `solana-test-validator --version` matches the `Cargo.toml` version
  - sh -c "$(curl -sSfL https://release.anza.xyz/<b>v2.0.10</b>/install)" -- to install a specific version

...

what you should see:

```
⠐ Initializing...
Identity: CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2
Genesis Hash: 9VRik4BdeNEoZdvAabsinfDjfXVLj2rCpCcLe6grVdem
Version: 2.0.10
Shred Version: 32893
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
WebSocket PubSub URL: ws://127.0.0.1:8900
⠙ 00:00:02 | Processed Slot: 2 | Confirmed Slot: 2 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 1 | ◎499.999995000                                                                                                                 account SysvarRecentB1ockHashes11111111111111111111 updated at slot 3!
account CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2 updated at slot 3!
account SysvarS1otHistory11111111111111111111111111 updated at slot 3!
account SysvarS1otHashes111111111111111111111111111 updated at slot 4!
account SysvarC1ock11111111111111111111111111111111 updated at slot 4!
account CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2 updated at slot 4!
```

... 

reference impls:
- https://github.com/clockwork-xyz/clockwork/tree/main/plugin
- https://github.com/solana-labs/solana-accountsdb-plugin-postgres/tree/master 
