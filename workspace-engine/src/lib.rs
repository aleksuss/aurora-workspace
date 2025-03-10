use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine_types::types::address::Address;
use aurora_engine_types::U256;
use aurora_workspace_utils::Contract;
use workspaces::types::{KeyType, SecretKey};
use workspaces::AccountId;

pub use aurora_workspace_utils::ContractId;
pub use contract::EngineContract;

pub mod contract;
pub mod operation;

pub mod types {
    pub use aurora_engine::proof::Proof;
    pub use aurora_workspace_types::AccountId;
    pub use aurora_workspace_types::ParseAccountError;
    pub use workspaces::types::KeyType;
    pub use workspaces::types::SecretKey;
    pub use workspaces::Account;
    pub use workspaces::Worker;

    pub mod input {
        pub use aurora_workspace_types::input::*;
    }

    pub mod network {
        pub use workspaces::network::Sandbox;
    }
}

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const OWNER_ACCOUNT_ID: &str = "aurora.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";

#[derive(Debug)]
pub struct EngineContractBuilder {
    code: Option<Vec<u8>>,
    chain_id: [u8; 32],
    owner_id: AccountId,
    prover_id: AccountId,
    custodian_address: Address,
    upgrade_delay_blocks: u64,
    ft_metadata: FungibleTokenMetadata,
}

impl EngineContractBuilder {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            code: None,
            chain_id: into_chain_id(AURORA_LOCAL_CHAIN_ID),
            owner_id: OWNER_ACCOUNT_ID.parse()?,
            prover_id: PROVER_ACCOUNT_ID.parse()?,
            custodian_address: Address::zero(),
            upgrade_delay_blocks: 1,
            ft_metadata: FungibleTokenMetadata::default(),
        })
    }

    pub fn with_code(mut self, code: Vec<u8>) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = into_chain_id(chain_id);
        self
    }

    pub fn with_owner_id(mut self, owner_id: &str) -> anyhow::Result<Self> {
        self.owner_id = owner_id.parse()?;
        Ok(self)
    }

    pub fn with_prover_id(mut self, prover_id: &str) -> anyhow::Result<Self> {
        self.prover_id = prover_id.parse()?;
        Ok(self)
    }

    pub fn with_custodian_address(mut self, address: &str) -> anyhow::Result<Self> {
        self.custodian_address = Address::decode(address).map_err(|e| anyhow::anyhow!({ e }))?;
        Ok(self)
    }

    pub fn with_upgrade_delay_blocks(mut self, upgrade_delay_blocks: u64) -> Self {
        self.upgrade_delay_blocks = upgrade_delay_blocks;
        self
    }

    pub fn with_ft_metadata(mut self, ft_metadata: FungibleTokenMetadata) -> Self {
        self.ft_metadata = ft_metadata;
        self
    }

    pub async fn deploy_and_init(self) -> anyhow::Result<EngineContract> {
        let worker = workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        let sk = SecretKey::from_random(KeyType::ED25519);
        let evm_account = worker
            .create_tla(self.owner_id.clone(), sk)
            .await?
            .into_result()?;
        let contract = Contract::deploy(evm_account, self.code.expect("WASM wasn't set")).await?;
        let contract = EngineContract::from(contract);

        contract
            .new(self.chain_id, self.owner_id, self.upgrade_delay_blocks)
            .transact()
            .await
            .map_err(|e| anyhow::anyhow!("error while initialize contract: {e}"))?;

        contract
            .new_eth_connector(
                self.prover_id,
                self.custodian_address.encode(),
                self.ft_metadata,
            )
            .transact()
            .await
            .map_err(|e| anyhow::anyhow!("error while initialize eth connector: {e}"))?;

        Ok(contract)
    }
}

fn into_chain_id(value: u64) -> [u8; 32] {
    let chain_id = U256::from(value);
    let mut result = [0; 32];
    chain_id.to_big_endian(&mut result);

    result
}
