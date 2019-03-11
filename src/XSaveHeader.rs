// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Layout of a `XSAVE` area.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct XSaveHeader
{
	/// Identifies the state components in the XSAVE area.
	///
	/// If a state component is not present in this bitmap, then it is not present in the legacy or extended area.
	pub XSTATE_BV: StateComponentBitmap,

	/// Identifies whether the extended region is compacted, and, if so, the format specifics as specified in Section 13.4.3 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	pub XCOMP_BV: CompactionStateComponentBitmap,

	reserved: [u64; 6],
}

impl XSaveHeader
{
	#[inline(always)]
	fn is_extended_region_compacted(&self) -> bool
	{
		self.XCOMP_BV.is_extended_region_compacted()
	}

	#[inline(always)]
	fn is_present(&self, state_component: StateComponent) -> bool
	{
		self.XSTATE_BV.is_present(state_component)
	}
}
