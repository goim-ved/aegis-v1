use ethers::prelude::*;
use std::sync::Arc;
use std::convert::TryFrom;

// Generate type-safe bindings
abigen!(
    AegisIDContract,
    "artifacts/contracts/AegisID.sol/AegisID.json"
);

abigen!(
    AegisWalletContract,
    "artifacts/contracts/AegisWallet.sol/AegisWallet.json"
);

abigen!(
    AegisRulesContract,
    "artifacts/contracts/AegisRules.sol/AegisRules.json"
);

#[derive(Clone)]
pub struct ChainClient {
    contract: AegisIDContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
    wallet_contract_template: AegisWalletContract<SignerMiddleware<Provider<Http>, LocalWallet>>, // Template to attach to dynamic addresses
}

impl ChainClient {
    pub async fn new(
        rpc_url: &str,
        private_key: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse()?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let wallet = wallet.with_chain_id(chain_id);

        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        let address: Address = contract_address.parse()?;
        let contract = AegisIDContract::new(address, client);

        let address: Address = contract_address.parse()?;
        let contract = AegisIDContract::new(address, client.clone());
        
        // We create a template instance just to have the type, 
        // effectively we will just use the client to connect to dynamic addresses later
        // or we can just instantiate new contracts on the fly in methods.
        // Let's use the second approach for cleaner code, but we need to keep the client accessible.
        // Actually, let's keep the client in the struct if we want.
        // A better way is to store the client in the struct.
        let wallet_contract_template = AegisWalletContract::new(Address::zero(), client);

        Ok(Self { contract, wallet_contract_template })
    }

    pub fn client(&self) -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
        self.contract.client()
    }

    pub async fn get_wallet_balance(&self, wallet_addr: &str) -> Result<String, Box<dyn std::error::Error>> {
        let addr: Address = wallet_addr.parse()?;
        // Use provider directly for ETH balance
        let balance = self.client().provider().get_balance(addr, None).await?;
        Ok(balance.to_string())
    }

    pub async fn fund_wallet(&self, wallet_addr: &str, amount_eth: &str) -> Result<String, Box<dyn std::error::Error>> {
         let to_addr: Address = wallet_addr.parse()?;
         let value = amount_eth.parse::<U256>()?; // Assuming amount is in wei for simplicity or use ethers utils
         // Actually let's assume input is standard decimal string, we might need parse_ether
         // For MVP, allow raw value or standard eth. Let's use parse_ether.
         let val_wei = ethers::utils::parse_ether(amount_eth)?;

         let tx = TransactionRequest::new()
            .to(to_addr)
            .value(val_wei);
        
        let pending = self.client().send_transaction(tx, None).await?;
        let receipt = pending.await?.ok_or("Transaction dropped")?;
        Ok(format!("{:?}", receipt.transaction_hash))
    }

    pub async fn set_agent_limit(&self, rules_addr: &str, agent_addr: &str, limit_eth: &str) -> Result<String, Box<dyn std::error::Error>> {
        let rules: Address = rules_addr.parse()?;
        let agent: Address = agent_addr.parse()?;
        let limit = ethers::utils::parse_ether(limit_eth)?;
        
        let contract = AegisRulesContract::new(rules, self.client());
        let call = contract.set_limit(agent, limit);
        let pending = call.send().await?;
        let receipt = pending.await?.ok_or("Transaction dropped")?;
        
        Ok(format!("{:?}", receipt.transaction_hash))
    }

    pub async fn execute_erc20(&self, wallet_addr: &str, token_addr: &str, to_addr: &str, amount: &str) -> Result<String, Box<dyn std::error::Error>> {
        let wallet: Address = wallet_addr.parse()?;
        let token: Address = token_addr.parse()?;
        let to: Address = to_addr.parse()?;
        // Assuming conversion is done upstream or raw value passed. 
        // Let's assume raw string value for tokens (decimals vary).
        let val = amount.parse::<U256>()?;

        // We need to attach AegisWallet template to the specific wallet address
        let contract = AegisWalletContract::new(wallet, self.client());
        let call = contract.execute_erc_20(token, to, val);
        let pending = call.send().await?;
        let receipt = pending.await?.ok_or("Transaction dropped")?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }

    pub async fn execute_native(&self, wallet_addr: &str, target_addr: &str, amount_eth: &str) -> Result<String, Box<dyn std::error::Error>> {
        let wallet: Address = wallet_addr.parse()?;
        let target: Address = target_addr.parse()?;
        let val = ethers::utils::parse_ether(amount_eth)?;
        let data = ethers::types::Bytes::new(); // Empty data for simple transfer

        let contract = AegisWalletContract::new(wallet, self.client());
        let call = contract.execute(target, val, data);
        let pending = call.send().await?;
        let receipt = pending.await?.ok_or("Transaction dropped")?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }
    
    pub async fn mint(&self, to: &str, uri: &str) -> Result<String, Box<dyn std::error::Error>> {
        let to_addr: Address = to.parse()?;
        let call = self.contract.mint(to_addr, uri.to_string());
        let pending_tx = call.send().await?;
        let receipt = pending_tx.await?.ok_or("Transaction dropped")?;
        
        Ok(format!("{:?}", receipt.transaction_hash))
    }
}
