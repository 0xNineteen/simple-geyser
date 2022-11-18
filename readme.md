- src/lib.rs: entrypoint 
- src/plugin.rs: main plugin code
- to run: `cargo build && solana-test-validator -r --geyser-plugin-config config.json`
- note: make sure `solana-test-validator --version` matches the `Cargo.toml` version
  - sh -c "$(curl -sSfL https://release.solana.com/<b>v1.14.1</b>/install)" -- to install a specific version

...

what you should see:

```
⠄ Initializing...
Identity: CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2
Genesis Hash: 9VRik4BdeNEoZdvAabsinfDjfXVLj2rCpCcLe6grVdem
Version: 1.10.34
Shred Version: 12622
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
⠚ 00:00:01 | Processed Slot: 1 | Confirmed Slot: 1 | Finalize
account SysvarRecentB1ockHashes11111111111111111111 updated at slot 2!
account CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2 updated at slot 2!
account SysvarS1otHistory11111111111111111111111111 updated at slot 2!
account SysvarS1otHashes111111111111111111111111111 updated at slot 3!
account SysvarC1ock11111111111111111111111111111111 updated at slot 3!
account CRCJ7zzd5SSmA8AJ9gbtv4QrYZ2zw4YKWa1MCDw1NTf2 updated at slot 3!
```

... 

reference impls:
- https://github.com/clockwork-xyz/clockwork/tree/main/plugin
- https://github.com/solana-labs/solana-accountsdb-plugin-postgres/tree/master 
- https://github.com/mwrites/solana-geyser-plugin-scaffold
