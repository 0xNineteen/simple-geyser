- src/lib.rs: entrypoint 
- src/plugin.rs: main plugin code
- to run: `cargo build && solana-test-validator -r --geyser-plugin-config config.json`

... 

reference impls:
- https://github.com/clockwork-xyz/clockwork/tree/main/plugin
- https://github.com/solana-labs/solana-accountsdb-plugin-postgres/tree/master 