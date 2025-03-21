use aurora_engine::parameters::{SubmitResult, TransactionStatus, WithdrawResult};
use aurora_workspace_types::{AccountId, Address, H256, U256};
use aurora_workspace_utils::results::{ExecutionResult, ViewResult};
use aurora_workspace_utils::transactions::{CallTransaction, ViewTransaction};
use aurora_workspace_utils::{impl_call_return, impl_view_return, Contract};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;

impl_call_return![
    (CallNew, Call::New),
    (CallNewEthConnector, Call::NewEthConnector),
    (CallFtTransfer, Call::FtTransfer),
    (CallDeposit, Call::Deposit),
    (
        CallSetEthConnectorContractData,
        Call::SetEthConnectorContractData
    ),
    (
        CallFactoryUpdateAddressVersion,
        Call::FactoryUpdateAddressVersion
    ),
    (CallRegisterRelayer, Call::RegisterRelayer),
    (CallRefundOnError, Call::RefundOnError),
    (CallFactoryUpdate, Call::FactoryUpdate),
    (CallFundXccSubAccount, Call::FundXccSubAccount),
    (CallFactorySetWNearAddress, Call::FactorySetWNearAddress),
    (CallDeployUpgrade, Call::DeployUpgrade),
    (CallResumePrecompiles, Call::ResumePrecompiles),
    (CallPausePrecompiles, Call::PausePrecompiles),
    (CallStageUpgrade, Call::StageUpgrade),
    (CallStateMigration, Call::StateMigration),
    (CallMintAccount, Call::MintAccount),
];

impl_call_return![
    (CallFtTransferCall => PromiseOrValue<U128>, Call::FtTransferCall, try_from),
    (CallStorageDeposit => StorageBalance, Call::StorageDeposit, json),
    (CallStorageUnregister => bool, Call::StorageUnregister, json),
    (CallStorageWithdraw => StorageBalance, Call::StorageWithdraw, json),
    (CallWithdraw => WithdrawResult, Call::Withdraw, borsh),
    (CallDeployCode => SubmitResult, Call::DeployCode, borsh),
    (CallDeployErc20Token => Address, Call::DeployErc20Token, borsh),
    (CallCall => SubmitResult, Call::Call, borsh),
    (CallSubmit => SubmitResult, Call::Submit, borsh),
    (CallFtOnTransfer => U128, Call::FtOnTransfer, json),
];

impl_view_return![
    (ViewFtTotalSupply => U128, View::FtTotalSupply, json),
    (ViewFtBalanceOf => U128, View::FtBalanceOf, json),
    (ViewStorageBalanceOf => StorageBalance, View::StorageBalanceOf, json),
    (ViewFtMetadata => FungibleTokenMetadata, View::FtMetadata, json),
    (ViewVersion => String, View::Version, borsh),
    (ViewOwner => AccountId, View::Owner, borsh),
    (ViewBridgeProver => AccountId, View::BridgeProver, borsh),
    (ViewChainId => AccountId, View::ChainId, borsh),
    (ViewUpgradeIndex => u64, View::UpgradeIndex, borsh),
    (ViewPausedPrecompiles => u32, View::PausedPrecompiles, borsh),
    (ViewBlockHash => H256, View::BlockHash, borsh_H256),
    (ViewCode => Vec<u8>, View::Code, borsh),
    (ViewBalance => U256, View::Balance, borsh_U256),
    (ViewNonce => U256, View::Nonce, borsh_U256),
    (ViewStorageAt => H256, View::StorageAt, borsh_H256),
    (ViewView => TransactionStatus, View::View, borsh),
    (ViewIsUsedProof => bool, View::IsUsedProof, borsh),
    (ViewFtTotalEthSupplyOnAurora => U256, View::FtTotalEthSupplyOnAurora, borsh_U256),
    (ViewFtBalanceOfEth => U256, View::FtBalanceOfEth, borsh_U256),
    (ViewErc20FromNep141 => Address, View::Erc20FromNep141, borsh),
    (ViewNep141FromErc20 => AccountId, View::Nep141FromErc20, borsh),
    (ViewPausedFlags => u8, View::PausedFlags, borsh)
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Call {
    New,
    NewEthConnector,
    DeployCode,
    DeployErc20Token,
    Call,
    Submit,
    RegisterRelayer,
    FtOnTransfer,
    Withdraw,
    Deposit,
    FtTransfer,
    FtTransferCall,
    StorageDeposit,
    StorageUnregister,
    StorageWithdraw,
    PausePrecompiles,
    StageUpgrade,
    DeployUpgrade,
    StateMigration,
    ResumePrecompiles,
    FactoryUpdate,
    FundXccSubAccount,
    FactorySetWNearAddress,
    SetEthConnectorContractData,
    FactoryUpdateAddressVersion,
    RefundOnError,
    MintAccount,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        match self {
            Call::New => "new",
            Call::NewEthConnector => "new_eth_connector",
            Call::DeployCode => "deploy_code",
            Call::DeployErc20Token => "deploy_erc20_token",
            Call::Call => "call",
            Call::Submit => "submit",
            Call::RegisterRelayer => "register_relayer",
            Call::FtOnTransfer => "ft_on_transfer",
            Call::Withdraw => "withdraw",
            Call::Deposit => "deposit",
            Call::FtTransfer => "ft_transfer",
            Call::FtTransferCall => "ft_transfer_call",
            Call::StorageDeposit => "storage_deposit",
            Call::StorageUnregister => "storage_unregister",
            Call::StorageWithdraw => "storage_withdraw",
            Call::PausePrecompiles => "pause_precompiles",
            Call::StageUpgrade => "stage_upgrade",
            Call::DeployUpgrade => "deploy_upgrade",
            Call::StateMigration => "state_migration",
            Call::ResumePrecompiles => "resume_precompiles",
            Call::FactoryUpdate => "factory_update",
            Call::FundXccSubAccount => "fund_xcc_sub_account",
            Call::FactorySetWNearAddress => "factory_set_wnear_address",
            Call::SetEthConnectorContractData => "set_eth_connector_contract_data",
            Call::FactoryUpdateAddressVersion => "factory_update_address_version",
            Call::RefundOnError => "refund_on_error",
            Call::MintAccount => "mint_account",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum View {
    Version,
    Owner,
    BridgeProver,
    ChainId,
    UpgradeIndex,
    PausedPrecompiles,
    BlockHash,
    Code,
    Balance,
    Nonce,
    StorageAt,
    View,
    IsUsedProof,
    FtTotalSupply,
    FtBalanceOf,
    FtBalanceOfEth,
    FtTotalEthSupplyOnAurora,
    FtMetadata,
    StorageBalanceOf,
    PausedFlags,
    Erc20FromNep141,
    Nep141FromErc20,
}

impl AsRef<str> for View {
    fn as_ref(&self) -> &str {
        match self {
            View::Version => "get_version",
            View::Owner => "get_owner",
            View::BridgeProver => "get_bridge_prover",
            View::ChainId => "get_chain_id",
            View::UpgradeIndex => "get_upgrade_index",
            View::PausedPrecompiles => "get_paused_precompiles",
            View::BlockHash => "get_block_hash",
            View::Code => "get_code",
            View::Balance => "get_balance",
            View::Nonce => "get_nonce",
            View::StorageAt => "get_storage_at",
            View::View => "get_view",
            View::IsUsedProof => "is_used_proof",
            View::FtTotalSupply => "ft_total_supply",
            View::FtBalanceOf => "ft_balance_of",
            View::FtBalanceOfEth => "ft_balance_of_eth",
            View::FtTotalEthSupplyOnAurora => "ft_total_eth_supply_on_aurora",
            View::FtMetadata => "ft_metadata",
            View::StorageBalanceOf => "storage_balance_of",
            View::PausedFlags => "get_paused_flags",
            View::Erc20FromNep141 => "get_erc20_from_nep141",
            View::Nep141FromErc20 => "get_nep141_from_erc20",
        }
    }
}
