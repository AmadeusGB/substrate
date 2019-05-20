#![deny(missing_docs, rust_2018_idioms)]

//! Slashing module

use parity_codec::Codec;
use sr_primitives::traits::{SimpleArithmetic, MaybeSerializeDebug};

/// Represents `generic` misconduct to be slashed
pub trait Misconduct {
	/// Amount calculated based on the misconduct and severity
	type Amount: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default;

	/// ...
	fn misconduct<B, S>(&self, balance: B, severity: S) -> Self::Amount
	where
		B: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default + Into<Self::Amount>,
		S: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default + Into<Self::Amount>;
}

/// ...
pub trait Slashing {

	/// Account id
	type AccountId: Codec + Copy + MaybeSerializeDebug + Default;

	/// Initial balance of the account
	type Balance: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default;

	/// Amount to slash
	type Amount: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default;

	/// Severity, based on number of misconducts in the given epoch
	type Severity: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default;

	/// Calculate amount to slash for account `who`
	// TODO(niklasad1): re-visit how to handle `severity` i.e., if it shall encapsulated in a struct or a parameter
	// to the `slash` function/method
	fn slash(
		&mut self,
		who: Self::AccountId,
		balance: Self::Balance,
		misconduct: impl Misconduct<Amount = Self::Amount>
	) -> Self::Amount;

	/// Signal epoch to reset severity
	// TODO(niklasad1): re-visit how to handle `severity` i.e., if it shall encapsulated in a struct or a parameter
	// to the `slash` function/method
	fn epoch_transition(&mut self);
}

#[cfg(test)]
mod tests {
	use super::*;

	pub struct Grandpa {
		severity: u32,
	}

	impl Slashing for Grandpa {
		type AccountId = u32;
		type Balance = u32;
		type Amount = u32;
		type Severity = u32;

		fn slash(
			&mut self,
			_who: Self::AccountId,
			balance: Self::Balance,
			m: impl Misconduct<Amount = Self::Amount>
		) -> Self::Amount {
			let new_balance = m.misconduct(balance, self.severity);
			self.severity += 1;
			new_balance
		}

		fn epoch_transition(&mut self) {
			self.severity = 0;
		}
	}

	enum GrandpaMisconduct {
		A,
		B,
		C,
	}

	impl Misconduct for GrandpaMisconduct {
		type Amount = u32;

		fn misconduct<B, S>(&self, balance: B, severity: S) -> Self::Amount
		where
			B: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default + Into<Self::Amount>,
			S: SimpleArithmetic + Codec + Copy + MaybeSerializeDebug + Default + Into<Self::Amount>
		{
			let x: Self::Amount = severity.into() * balance.into();

			match &self {
				GrandpaMisconduct::A => x + 0,
				GrandpaMisconduct::B => x + 1,
				GrandpaMisconduct::C => x + 2,
			}
		}
	}

	#[test]
    fn it_works() {
		let mut g = Grandpa { severity: 1 };
        assert_eq!(100, g.slash(0, 100, GrandpaMisconduct::A));
        assert_eq!(2, g.severity);
    }
}
