//! Extensions for BanksClient.
//!
//! This module provides utilities for working with BanksClient
//! in tests. While some functionality overlaps with Solana's
//! ProgramTestBanksClientExt, this implementation adds additional
//! methods and is designed to work seamlessly with solana-accountgen.

use solana_banks_client::BanksClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    hash::Hash,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::io;
use std::time::{Duration, Instant};

/// Extension trait for BanksClient to add useful testing methods.
#[async_trait::async_trait]
pub trait BanksClientExt {
    /// Get a new latest blockhash, similar to RpcClient::get_latest_blockhash()
    ///
    /// Note: This functionality is similar to Solana's ProgramTestBanksClientExt,
    /// but is included here for convenience and to provide a complete API.
    async fn get_new_latest_blockhash(&mut self, blockhash: &Hash) -> io::Result<Hash>;
    
    /// Process a transaction and wait for confirmation.
    ///
    /// This method processes a transaction and returns an error if the transaction fails.
    /// It's a convenience wrapper around BanksClient::process_transaction that provides
    /// better error handling.
    async fn process_transaction_with_preflight(
        &mut self,
        transaction: Transaction,
    ) -> io::Result<()>;
}

#[async_trait::async_trait]
impl BanksClientExt for BanksClient {
    async fn get_new_latest_blockhash(&mut self, blockhash: &Hash) -> io::Result<Hash> {
        let mut num_retries = 0;
        let start = Instant::now();
        while start.elapsed().as_secs() < 5 {
            let new_blockhash = self.get_latest_blockhash().await?;
            if new_blockhash != *blockhash {
                return Ok(new_blockhash);
            }
            
            tokio::time::sleep(Duration::from_millis(200)).await;
            num_retries += 1;
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Unable to get new blockhash after {}ms (retried {} times), stuck at {}",
                start.elapsed().as_millis(),
                num_retries,
                blockhash
            ),
        ))
    }
    
    async fn process_transaction_with_preflight(
        &mut self,
        transaction: Transaction,
    ) -> io::Result<()> {
        self.process_transaction(transaction.clone()).await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Transaction failed: {:?}", e),
            )
        })?;
        
        Ok(())
    }
} 