use {
    lazy_static::lazy_static,
    solana_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod instructions_sysvar_enabled {
    solana_sdk::declare_id!("EnvhHCLvg55P7PDtbvR1NwuTuAeodqpusV3MR5QEK8gs");
}

pub mod consistent_recent_blockhashes_sysvar {
    solana_sdk::declare_id!("3h1BQWPDS5veRsq6mDBWruEpgPxRJkfwGexg5iiQ9mYg");
}

pub mod deprecate_rewards_sysvar {
    solana_sdk::declare_id!("GaBtBJvmS4Arjj5W1NmFcyvPjsHN38UGYDq2MDwbs9Qu");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("4RWNif6C2WCNiKVW7otP4G7dkmkHGyKQWRpuZ1pxKU5m");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        solana_sdk::declare_id!("DT4n6ABDqs6w4bnfwrXT9rsprcPf6cdDga1egctaPkLC");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                solana_sdk::declare_id!("BzBBveUDymEYoYzcMWNQCx3cd4jQs7puaVFHLtsbB6fm");
            }
            pub mod enable {
                solana_sdk::declare_id!("7XRJcS5Ud5vxGB54JbK9N2vBZVwnwdBNeJW1ibRgD9gx");
            }
        }
    }
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("E5JiFDQCwyC6QfT9REFyMpfK2mHcmv1GUDySU1Ue7TYv");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("4kpdyrcj5jS47CZb2oJGfVxjYbsMm2Kx97gFyZrxxwXz");
}

pub mod filter_stake_delegation_accounts {
    solana_sdk::declare_id!("GE7fRxmW46K6EmCD9AMZSbnaJ2e3LfqCZzdHi9hmYAgi");
}

pub mod bpf_loader_upgradeable_program {
    solana_sdk::declare_id!("FbhK8HN9qvNHvJcoFVHAEUCNkagHvu7DTWzdnLuVQ5u4");
}

pub mod stake_program_v3 {
    solana_sdk::declare_id!("Ego6nTu7WsBcZBvVqJQKp6Yku2N3mrfG8oYCfaLZkAeK");
}

pub mod require_custodian_for_locked_stake_authorize {
    solana_sdk::declare_id!("D4jsDcXaqdW8tDAWn8H4R25Cdns2YwLneujSL1zvjW6R");
}

pub mod spl_token_v2_self_transfer_fix {
    solana_sdk::declare_id!("BL99GYhdjjcv6ys22C9wPgn2aTVERDbPHHo4NbS3hgp7");
}

pub mod warp_timestamp_again {
    solana_sdk::declare_id!("GvDsGDkH5gyzwpDhxNixx8vtx1kwYHH13RiNAPw27zXb");
}

pub mod check_init_vote_data {
    solana_sdk::declare_id!("3ccR6QpxGYsAbWyfevEtBNGfWV4xBffxRj2tD6A9i39F");
}

pub mod check_program_owner {
    solana_sdk::declare_id!("5XnbR5Es9YXEARRuP6mdvoxiW3hx5atNNeBmwVd8P3QD");
}

pub mod require_stake_for_gossip {
    solana_sdk::declare_id!("6oNzd5Z3M2L1xo4Q5hoox7CR2DuW7m1ETLWH5jHJthwa");
}

pub mod cpi_data_cost {
    solana_sdk::declare_id!("Hrg5bXePPGiAVWZfDHbvjqytSeyBDPAGAQ7v6N5i4gCX");
}

pub mod upgradeable_close_instruction {
    solana_sdk::declare_id!("FsPaByos3gA9bUEhp3EimQpQPCoSvCEigHod496NmABQ");
}

pub mod sysvar_via_syscall {
    solana_sdk::declare_id!("7411E6gFQLDhQkdRjmpXwM1hzHMMoYQUjHicmvGPC1Nf");
}

pub mod enforce_aligned_host_addrs {
    solana_sdk::declare_id!("6Qob9Z4RwGdf599FDVCqsjuKjR8ZFR3oVs2ByRLWBsua");
}

pub mod set_upgrade_authority_via_cpi_enabled {
    solana_sdk::declare_id!("GQdjCCptpGECG7QfE35hKTAopB1umGoSrdKfax2VmZWy");
}

pub mod update_data_on_realloc {
    solana_sdk::declare_id!("BkPcYCrwHXBoTsv9vMhiRF9gteZmDj3Uwisz9CDjoMKp");
}

pub mod keccak256_syscall_enabled {
    solana_sdk::declare_id!("7Ua8mFtahVfA3WCY9LoXDAJJdvJRJHckvSSr1dD8FTWc");
}

pub mod stake_program_v4 {
    solana_sdk::declare_id!("Dc7djyhP9aLfdq2zktpvskeAjpG56msCU1yexpxXiWZb");
}

pub mod memory_ops_syscalls {
    solana_sdk::declare_id!("ENQi37wsVhTvFz2gUiZAAbqFEWGN2jwFsqdEDTE8A4MU");
}

pub mod secp256k1_recover_syscall_enabled {
    solana_sdk::declare_id!("6RvdSWHh8oh72Dp7wMTS2DBkf3fRPtChfNrAo3cZZoXJ");
}

pub mod add_missing_program_error_mappings {
    solana_sdk::declare_id!("3QEUpjhgPEt92nz3Mqf6pABkHPGCQwSvKtyGMq4SuQyL");
}

pub mod system_transfer_zero_check {
    solana_sdk::declare_id!("BrTR9hzw4WBGFP65AJMbpAo64DcA3U6jdPSga9fMV5cS");
}

pub mod dedupe_config_program_signers {
    solana_sdk::declare_id!("8kEuAshXLsgkUEdcFVLqrjCGGHVWFW99ZZpxvAzzMtBp");
}

pub mod deterministic_shred_seed_enabled {
    solana_sdk::declare_id!("FjSRMpFe7mofQ3WrEMT7Smjk2sME1XdAoRxcv55V6M44");
}

pub mod verify_tx_signatures_len {
    solana_sdk::declare_id!("EVW9B5xD9FFK7vw1SBARwMA4s5eRo5eKJdKpsBikzKBz");
}

pub mod vote_stake_checked_instructions {
    solana_sdk::declare_id!("BcWknVcgvonN8sL4HE4XFuEVgfcee5MwxWPAgP6ZV89X");
}

pub mod updated_verify_policy {
    solana_sdk::declare_id!("k15tVxtkgsmo7dy6iJ56N5hBCxuQAtqRgYwoTDuwbia");
}

pub mod neon_evm_compute_budget {
    solana_sdk::declare_id!("GLrVvDPkQi5PMYUrsYWT9doZhSHr1BVZXqj5DbFps3rS");
}

pub mod rent_for_sysvars {
    solana_sdk::declare_id!("BKCPBQQBZqggVnFso5nQ8rQ4RwwogYwjuUt9biBjxwNF");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    solana_sdk::declare_id!("DhsYfRjxfnh2g7HKJYSzT79r74Afa1wbHkAgHndrA1oy");
}

pub mod stop_verify_mul64_imm_nonzero {
    solana_sdk::declare_id!("EHFwHg2vhwUb7ifm7BuY9RMbsyt1rS1rUii7yeDJtGnN");
}

pub mod start_verify_shift32_imm {
    solana_sdk::declare_id!("CqvdhqAYMc6Eq6tjW3H42Qg39TK2SCsL8ydMsC363PRp");
}

pub mod merge_nonce_error_into_system_error {
    solana_sdk::declare_id!("21AWDosvp3pBamFW91KB35pNoaoZVTM7ess8nr2nt53B");
}

pub mod spl_token_v2_set_authority_fix {
    solana_sdk::declare_id!("FToKNBYyiF4ky9s8WsmLBXHCht17Ek7RXaLZGHzzQhJ1");
}

pub mod stake_merge_with_unmatched_credits_observed {
    solana_sdk::declare_id!("meRgp4ArRPhD3KtCY9c5yAf2med7mBLsjKTPeVUHqBL");
}

pub mod mem_overlap_fix {
    solana_sdk::declare_id!("vXDCFK7gphrEmyf5VnKgLmqbdJ4UxD2eZH1qbdouYKF");
}

pub mod close_upgradeable_program_accounts {
    solana_sdk::declare_id!("EQMtCuSAkMVF9ZdhGuABtgvyXJLtSRF5AQKv1RNsrhj7");
}

pub mod stake_program_advance_activating_credits_observed {
    solana_sdk::declare_id!("SAdVFw3RZvzbo6DvySbSdBnHN4gkzSTH9dSxesyKKPj");
}

pub mod demote_program_write_locks {
    solana_sdk::declare_id!("3E3jV7v9VcdJL8iYZUMax9DiDno8j7EWUVbhm9RtShj2");
}

pub mod allow_native_ids {
    solana_sdk::declare_id!("GVnDbNkECwrzLM7aVBGWpBYo3yH1ACaXB4ottNX8pedZ");
}

pub mod check_seed_length {
    solana_sdk::declare_id!("8HYXgkoKGreAMA3MfJkdjbKNVbfZRQP3jqFpa7iqN4v7");
}

pub mod fix_write_privs {
    solana_sdk::declare_id!("7Tr5C1tdcCeBVD8jxtHYnvjL1DGdFboYBHCJkEFdenBb");
}

pub mod reduce_required_deploy_balance {
    solana_sdk::declare_id!("EBeznQDjcPG8491sFsKZYBi5S5jTVXMpAKNDJMQPS2kq");
}

pub mod stakes_remove_delegation_if_inactive {
    solana_sdk::declare_id!("HFpdDDNQjvcXnXKec697HDDsyk6tFoWS2o8fkxuhQZpL");
}

pub mod send_to_tpu_vote_port {
    solana_sdk::declare_id!("C5fh68nJ7uyKAuYZg2x9sEQ5YrVf3dkW6oojNBSc3Jvo");
}

pub mod optimize_epoch_boundary_updates {
    solana_sdk::declare_id!("265hPS8k8xJ37ot82KEgjRunsUp5w4n4Q4VwwiN9i9ps");
}

pub mod tx_wide_compute_cap {
    solana_sdk::declare_id!("5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9");
}

pub mod gate_large_block {
    solana_sdk::declare_id!("2ry7ygxiYURULZCrypHhveanvP5tzZ4toRwVp89oCNSj");
}

pub mod remove_native_loader {
    solana_sdk::declare_id!("HTTgmruMYRZEntyL3EdCDdnS6e4D5wRq1FA7kQsb66qq");
}

pub mod return_data_syscall_enabled {
    solana_sdk::declare_id!("DwScAzPUjuv65TMbDnFY7AgwmotzWy3xpEJMXM3hZFaB");
}

pub mod sol_log_data_syscall_enabled {
    solana_sdk::declare_id!("6uaHcKPGUy4J7emLBgUTeufhJdiwhngW6a1R9B7c2ob9");
}

pub mod ed25519_program_enabled {
    solana_sdk::declare_id!("6ppMXNYLhVd7GcsZ5uV11wQEW7spppiMVfqQv5SXhDpX");
}

pub mod requestable_heap_size {
    solana_sdk::declare_id!("CCu4boMmfLuqcmfTLPHQiUo22ZdUsXjgzPAURYaWt1Bw");
}

pub mod add_compute_budget_program {
    solana_sdk::declare_id!("4d5AKtxoh93Dwm1vHXUU3iRATuMndx1c431KgT2td52r");
}

pub mod reject_deployment_of_unresolved_syscalls {
    solana_sdk::declare_id!("DqniU3MfvdpU3yhmNF1RKeaM5TZQELZuyFGosASRVUoy");
}

pub mod reject_section_virtual_address_file_offset_mismatch {
    solana_sdk::declare_id!("5N4NikcJLEiZNqwndhNyvZw15LvFXp1oF7AJQTNTZY5k");
}

pub mod reject_all_elf_rw {
    solana_sdk::declare_id!("DeMpxgMq51j3rZfNK2hQKZyXknQvqevPSFPJFNTbXxsS");
}

pub mod spl_token_v3_3_0_release {
    solana_sdk::declare_id!("Ftok2jhqAqxUWEiCVRrfRs9DPppWP8cgTB7NQNKL88mS");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    solana_sdk::declare_id!("7txXZZD6Um59YoLMF7XUNimbMjsqsWhc7g2EniiTrmp1");
}

pub mod evict_invalid_stakes_cache_entries {
    solana_sdk::declare_id!("EMX9Q7TVFAmQ9V1CggAkhMzhXSg8ECp7fHrWQX2G1chf");
}

pub mod spl_associated_token_account_v1_0_4 {
    solana_sdk::declare_id!("FaTa4SpiaSNH44PGC4z8bnGVTkSRYaWvrBs3KTu8XQQq");
}

<<<<<<< HEAD
=======
pub mod reject_vote_account_close_unless_zero_credit_epoch {
    solana_sdk::declare_id!("ALBk3EWdeAg2WAGf6GPDUf1nynyNqCdEVmgouG7rpuCj");
}

pub mod add_get_processed_sibling_instruction_syscall {
    solana_sdk::declare_id!("CFK1hRCNy8JJuAAY8Pb2GjLFNdCThS2qwZNe3izzBMgn");
}

pub mod bank_tranaction_count_fix {
    solana_sdk::declare_id!("Vo5siZ442SaZBKPXNocthiXysNviW4UYPwRFggmbgAp");
}

>>>>>>> bd1850df2 (Return actual committed transactions from process_transactions() (#22802))
lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (instructions_sysvar_enabled::id(), "instructions sysvar"),
        (consistent_recent_blockhashes_sysvar::id(), "consistent recentblockhashes sysvar"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (spl_token_v2_multisig_fix::id(), "spl-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (bpf_loader_upgradeable_program::id(), "upgradeable bpf loader"),
        (stake_program_v3::id(), "solana_stake_program v3"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (spl_token_v2_self_transfer_fix::id(), "spl-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (check_program_owner::id(), "limit programs to operating on accounts owned by itself"),
        (require_stake_for_gossip::id(), "require stakes for propagating crds values through gossip #15561"),
        (cpi_data_cost::id(), "charge the compute budget for data passed via CPI"),
        (upgradeable_close_instruction::id(), "close upgradeable buffer accounts"),
        (sysvar_via_syscall::id(), "provide sysvars via syscalls"),
        (enforce_aligned_host_addrs::id(), "enforce aligned host addresses"),
        (set_upgrade_authority_via_cpi_enabled::id(), "set upgrade authority instruction via cpi calls for upgradable programs"),
        (update_data_on_realloc::id(), "Retain updated data values modified after realloc via CPI"),
        (keccak256_syscall_enabled::id(), "keccak256 syscall"),
        (stake_program_v4::id(), "solana_stake_program v4"),
        (memory_ops_syscalls::id(), "add syscalls for memory operations"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (add_missing_program_error_mappings::id(), "add missing program error mappings"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (deterministic_shred_seed_enabled::id(), "deterministic shred seed"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (updated_verify_policy::id(), "Update verify policy"),
        (neon_evm_compute_budget::id(), "bump neon_evm's compute budget"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (stop_verify_mul64_imm_nonzero::id(), "Sets rbpf vm config verify_mul64_imm_nonzero to false"),
        (start_verify_shift32_imm::id(), "sets rbpf vm config verify_shift32_imm to true"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (spl_token_v2_set_authority_fix::id(), "spl-token set_authority fix"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (mem_overlap_fix::id(), "Memory overlap fix"),
        (close_upgradeable_program_accounts::id(), "enable closing upgradeable program accounts"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (allow_native_ids::id(), "allow native program ids in program derived addresses"),
        (check_seed_length::id(), "Check program address seed lengths"),
        (fix_write_privs::id(), "fix native invoke write privileges"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (send_to_tpu_vote_port::id(), "Send votes to the tpu vote port"),
        (optimize_epoch_boundary_updates::id(), "Optimize epoch boundary updates"),
        (tx_wide_compute_cap::id(), "Transaction wide compute cap"),
        (gate_large_block::id(), "validator checks block cost against max limit in realtime, reject if exceeds."),
        (remove_native_loader::id(), "Remove support for the native loader"),
        (return_data_syscall_enabled::id(), "enable sol_{set,get}_return_data syscall"),
        (sol_log_data_syscall_enabled::id(), "enable sol_log_data syscall"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (reject_deployment_of_unresolved_syscalls::id(), "Reject deployment of programs with unresolved syscall symbols"),
        (reject_section_virtual_address_file_offset_mismatch::id(), "enforce section virtual addresses and file offsets in ELF to be equal"),
        (reject_all_elf_rw::id(), "reject all read-write data in program elfs"),
        (spl_token_v3_3_0_release::id(), "spl-token v3.3.0 release"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        (spl_associated_token_account_v1_0_4::id(), "SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
<<<<<<< HEAD
=======
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_tranaction_count_fix::id(), "Fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
>>>>>>> bd1850df2 (Return actual committed transactions from process_transactions() (#22802))
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }
}
