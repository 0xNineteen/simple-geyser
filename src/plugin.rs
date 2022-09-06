use { 
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult
    },
    solana_program::pubkey::Pubkey
};

#[derive(Debug)]
pub struct SimplePlugin { }

impl Default for SimplePlugin {
    fn default() -> Self {
        SimplePlugin {}
    }
}

impl GeyserPlugin for SimplePlugin {

    fn name(&self) -> &'static str {
        "simple-geyser"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {

        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(account_info) => {
                account_info
            }
        };

        let pk = Pubkey::new(account_info.pubkey);
        println!("account {:#?} updated at slot {}!", pk, slot);

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true // process account changes
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false // dont process new txs
    }

}

