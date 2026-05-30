// Copyright 2026 The Libernet Team
// SPDX-License-Identifier: Apache-2.0

mod avm;

mod starkom {
    pub mod avm {
        include!(concat!(env!("OUT_DIR"), "/starkom.avm.rs"));
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/starkom.avm.v1.rs"));
        }
    }
}

pub use avm::*;
pub use starkom::avm::v1 as proto;
