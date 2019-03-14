// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// A state component bitmap.
///
/// Can be used to work with the extended control register, `XCR0`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StateComponentBitmap(pub(crate) u64);

impl StateComponentBitmap
{
	/// Read the value of the register `XCR0`.
	///
	/// Will only work if the Operating System has set bit 18 in the register `CR4.OSXSAVE`, otherwise usage will cause an invalid-opcode exception (`#UD`).
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "xsave"))]
	pub fn save_from_xcr0() -> Self
	{
		Self(unsafe { _xgetbv(_XCR_XFEATURE_ENABLED_MASK) })
	}

	/// Read the value of the register `XCR0` AND'd with the current value of the `XINUSE` bitmap.
	///
	/// This can be used in conjunction with `xsaveopt` to reduce the amount of data that needs to be saved.
	///
	/// Will only work if the Operating System has set bit 18 in the register `CR4.OSXSAVE`, otherwise usage will cause an invalid-opcode exception (`#UD`).
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "xsave", target_feature = "xsaveopt"))]
	pub fn save_from_xcr0_with_init_optimization() -> Self
	{
		Self(unsafe { _xgetbv(1) })
	}

	/// Write this value into the `register`.
	///
	/// Only allowed for kernel-mode code; use in other modes will cause a general-protected fault (`#GP`).
	///
	/// Will only work if the Operating System has set bit 18 in the register `CR4.OSXSAVE`, otherwise usage will cause an invalid-opcode exception (`#UD`).
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "xsave"))]
	#[inline(always)]
	pub fn restore_to_xcr0(self)
	{
		unsafe { _xsetbv(_XCR_XFEATURE_ENABLED_MASK, self.0) }
	}

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
