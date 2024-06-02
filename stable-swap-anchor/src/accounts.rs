//! Accounts structs for StableSwap.

use anchor_lang::prelude::*;











/// Accounts for a [crate::set_fee_account] instruction.
#[derive(Accounts, Clone)]
pub struct SetFeeAccount<'info> {
    /// The context of the admin user
    pub admin_ctx: AdminUserContext<'info>,
    /// The new token account for fees
    pub fee_account: AccountInfo<'info>,
}

/// Accounts for a [crate::apply_new_admin] instruction.
#[derive(Accounts, Clone)]
pub struct CommitNewAdmin<'info> {
    /// The context of the admin user.
    pub admin_ctx: AdminUserContext<'info>,
    /// The account of the new admin.
    pub new_admin: AccountInfo<'info>,
}

// --------------------------------
// Various accounts
// --------------------------------

/// Token accounts for initializing a [crate::SwapInfo].
#[derive(Accounts, Clone)]
pub struct InitToken<'info> {
    /// The token account for the pool's reserves of this token.
    pub reserve: AccountInfo<'info>,
    /// The token account for the fees associated with the token.
    pub fees: AccountInfo<'info>,
    /// The mint of the token.
    pub mint: AccountInfo<'info>,
}

/// Token accounts for a [crate::swap] instruction.
#[derive(Accounts, Clone)]
pub struct SwapToken<'info> {
    /// The token account associated with the user.
    pub user: AccountInfo<'info>,
    /// The token account for the pool's reserves of this token.
    pub reserve: AccountInfo<'info>,
}

/// Token accounts for the output of a StableSwap instruction.
#[derive(Accounts, Clone)]
pub struct SwapOutput<'info> {
    /// The token accounts of the user and the token.
    pub user_token: SwapToken<'info>,
    /// The token account for the fees associated with the token.
    pub fees: AccountInfo<'info>,
}

/// Accounts for an instruction that interacts with the swap.
#[derive(Accounts, Clone)]
pub struct SwapUserContext<'info> {
    /// The spl_token program.
    pub token_program: AccountInfo<'info>,
    /// The authority of the swap.
    pub swap_authority: AccountInfo<'info>,
    /// The authority of the user.
    #[account(signer)]
    pub user_authority: AccountInfo<'info>,
    /// The swap.
    pub swap: AccountInfo<'info>,
}

/// Accounts for an instruction that requires admin permission.
#[derive(Accounts, Clone)]
pub struct AdminUserContext<'info> {
    /// The public key of the admin account.
    ///
    /// *Note: must be a signer.*
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    /// The swap.
    pub swap: AccountInfo<'info>,
}
