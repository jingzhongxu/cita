// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Basic account type -- the decoded RLP from the state trie.
#![rustfmt_skip]

use rlp::*;
use util::{U256, H256};

/// Basic account type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicAccount {
    /// Nonce of the account.
    pub nonce: U256,
    /// Storage root of the account.
    pub storage_root: H256,
    /// Code hash of the account.
    pub code_hash: H256,
    /// ABI hash of the account.
    pub abi_hash: H256,
}

impl Encodable for BasicAccount {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4).append(&self.nonce)
                       .append(&self.storage_root)
                       .append(&self.code_hash)
                       .append(&self.abi_hash);
    }
}

impl Decodable for BasicAccount {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(BasicAccount {
               nonce: rlp.val_at(0)?,
               storage_root: rlp.val_at(1)?,
               code_hash: rlp.val_at(2)?,
               abi_hash: rlp.val_at(3)?,               
           })
    }
}
