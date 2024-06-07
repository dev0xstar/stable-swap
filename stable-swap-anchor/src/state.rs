//! State structs for swaps.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use std::ops::Deref;

/// StableSwap account wrapper for Anchor programs.
///
/// *For more info, see [stable_swap_client::state::SwapInfo].*
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwapInfo(stable_swap_client::state::SwapInfo);

impl SwapInfo {
    /// The length, in bytes, of the packed representation
    pub const LEN: usize = stable_swap_client::state::SwapInfo::LEN;

    /// Computes the minimum rent exempt balance of a [SwapInfo].
    pub fn minimum_rent_exempt_balance() -> Result<u64> {
        Ok(Rent::get()?.minimum_balance(Self::LEN))
    }
}

impl Owner for SwapInfo {
    fn owner() -> Pubkey {
        crate::ID
    }
}




