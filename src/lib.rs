// RGB standard library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[macro_use]
extern crate derive_wrapper;
extern crate chrono;
extern crate lightning_invoice;
extern crate regex;
#[macro_use]
extern crate lnpbp;

pub(in crate::bin::stashd) mod stashd;

mod contracts;
mod error;

pub use contracts::*;
pub use error::BootstrapError;

// Re-exports
pub use lnpbp::rgb as std;
