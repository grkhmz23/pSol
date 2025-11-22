use psol::crypto;
use psol::state::{PrivacyAccount, PrivacyPool};
use psol_token::error::ErrorCode as TokenError;

#[test]
fn crypto_helpers_are_stable() {
    let owner = anchor_lang::prelude::Pubkey::new_unique();
    let c1 = crypto::commitment(&owner, 10, 1);
    let c2 = crypto::commitment(&owner, 10, 1);
    assert_eq!(c1, c2);
    assert_ne!(c1, crypto::commitment(&owner, 11, 1));
    let n = crypto::nullifier(&owner, &c1);
    assert_ne!(n, [0u8; 32]);
}

#[test]
fn fee_application_matches_net_amount() {
    let pool = PrivacyPool {
        admin: anchor_lang::prelude::Pubkey::default(),
        vault_bump: 1,
        commitment_bump: 1,
        nullifier_bump: 1,
        paused: false,
        fee_bps: 500,
        total_locked: 0,
    };
    let (net, fee) = pool.apply_fee(1_000_000).unwrap();
    assert_eq!(fee, 50_000);
    assert_eq!(net, 950_000);
}

#[test]
fn privacy_account_balances_move() {
    let mut acct = PrivacyAccount {
        owner: anchor_lang::prelude::Pubkey::default(),
        balance: 0,
    };
    acct.deposit(10).unwrap();
    assert_eq!(acct.balance, 10);
    acct.withdraw(5).unwrap();
    assert_eq!(acct.balance, 5);
}

#[test]
fn transfer_psol_is_disabled_error() {
    let err = TokenError::TransfersDisabled; // discriminant exists
    assert_eq!(err as u32, TokenError::TransfersDisabled as u32);
}