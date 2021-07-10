use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow (Alice core account)
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer (Alice temp token X account)
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through (Alice token Y account)
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade. (X Mint account)
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program (the program itself)
  InitEscrow {
    /// The amount party A expects to receive of token Y
    amount: u64
  }
}
