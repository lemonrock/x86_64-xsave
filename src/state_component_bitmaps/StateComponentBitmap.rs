// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// A state component bitmap.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StateComponentBitmap(pub(crate) u64);

impl StateComponentBitmap
{
	/// Is present?
	#[inline(always)]
	pub fn set_is_present(&mut self, state_component: StateComponent)
	{
		let bit = state_component.bit();

		self.0 |= 1 << (bit as u64);
	}

	/// Is present?
	#[inline(always)]
	pub fn is_present(self, state_component: StateComponent) -> bool
	{
		self.bit_set(state_component.bit())
	}

	#[inline(always)]
	fn bit_set(self, bit: u8) -> bool
	{
		self.0 & (1 << (bit as u64)) != 0
	}

	#[inline(always)]
	fn bits_are_zero(self, mask: u64) -> bool
	{
		(self.0 & mask) == 0
	}
}
