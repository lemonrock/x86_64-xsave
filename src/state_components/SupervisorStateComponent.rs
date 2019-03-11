// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// A supervisor (operating system) state component.
///
/// Support can be checked in the `IA32_XSS` `is_extended_region_compacted` register.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SupervisorStateComponent
{
	/// The state component used for the Intel Processor Trace MSRs (`PT` state).
	PT = 8,

	/// The state component used for an `MSR` used to control hardware duty cycling (`HDC` state).
	///
	/// See Section 13.5.8 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	HDC = 13,
}

impl SupervisorStateComponent
{
	/// To a state component.
	#[inline(always)]
	pub const fn to_state_component(self) -> StateComponent
	{
		StateComponent
		{
			supervisor: self,
		}
	}

	/// Query size using `CPUID` of this supervisor state component.
	#[inline(always)]
	pub fn sizing(self) -> Option<StateComponentSizing>
	{
		StateComponentSizing::sizing(self as u8)
	}
}
