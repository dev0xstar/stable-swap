//! State structs for swaps.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use std::ops::Deref;

/// StableSwap account wrapper for Anchor programs.
///
/// *For more info, see [stable_swap_client::state::SwapInfo].*
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwapInfo(stable_swap_client::state::SwapInfo);

