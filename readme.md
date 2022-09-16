- src/lib.rs: entrypoint 
- src/plugin.rs: main plugin code
- to run: `cargo build && solana-test-validator -r --geyser-plugin-config config.json`
- note: make sure `solana-test-validator --version` matches the `Cargo.toml` version

... 

reference impls:
- https://github.com/clockwork-xyz/clockwork/tree/main/plugin
- https://github.com/solana-labs/solana-accountsdb-plugin-postgres/tree/master 
