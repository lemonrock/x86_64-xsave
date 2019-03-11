// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// All bits in this should be 0 (false) if the processor does not support the compaction extensions to the `XSAVE` feature set.
///
/// See Sections 13.4.3 and 13.8 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CompactionStateComponentBitmap(StateComponentBitmap);

impl Into<StateComponentBitmap> for CompactionStateComponentBitmap
{
	#[inline(always)]
	fn into(self) -> StateComponentBitmap
	{
		self.state_component_bitmap()
	}
}

impl CompactionStateComponentBitmap
{
	/// The underlying state component bitmap.
	#[inline(always)]
	pub fn state_component_bitmap(self) -> StateComponentBitmap
	{
		self.0
	}

	/// Is the extended region compacted?
	#[inline(always)]
	pub fn is_extended_region_compacted(self) -> bool
	{
		self.state_component_bitmap().bit_set(63)
	}

	/// Is the state component enabled in the compacted region?
	#[inline(always)]
	pub fn is_present(self, state_component: StateComponent) -> bool
	{
		self.state_component_bitmap().is_present(state_component)
	}

	#[inline(always)]
	pub(crate) fn bit_set(self, i: u8) -> bool
	{
		self.state_component_bitmap().bit_set(i)
	}

	#[inline(always)]
	pub(crate) fn bits_are_zero(self, mask: u64) -> bool
	{
		self.state_component_bitmap().bits_are_zero(mask)
	}
}
