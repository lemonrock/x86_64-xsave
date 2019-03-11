// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// This structure is not valid for state components `0` (`x87` state) or `1` (`SSE` state).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateComponentSizing
{
	/// Byte offset from base address of `XSaveArea` if uncompacted.
	pub uncompacted_byte_offset: usize,

	/// Size of state component in bytes; never zero if obtained via `StateComponent.sizing()`, `SizingQueryableUserStateComponent.sizing()`, `SupervisorStateComponent.sizing()` or `StateComponentSizing::sizing()`.
	pub size: usize,

	/// Which register supports this state component?
	pub supported_in: SupportedIn,

	/// If compacted, does this state component require alignment to 64 bytes?
	///
	/// * If true, this extended state component is located on the next 64-byte boundary following the preceding state component.
	/// * If false, it is located immediately following the preceding state component.
	pub requires_alignment_if_compacted: bool,
}

impl StateComponentSizing
{
	/// Query size using `CPUID` of the state component for `bit`.
	///
	/// `bit` must be between 2 and 62 inclusive.
	#[inline(always)]
	pub fn sizing(bit: u8) -> Option<StateComponentSizing>
	{
		debug_assert!(bit >= 2, "bit `{}` is 0 or 1", bit);
		debug_assert_ne!(bit, 63, "bit is 63");
		debug_assert!(bit < 64, "Only 64 bits are supported, so not `{}`", bit);

		if cfg!(target_arch = "x86_64")
		{
			let values = match ExtendedStateInformation::eax_extended_state_information(bit as u32)
			{
				None => return None,
				Some(values) => values,
			};

			let eax = values.eax;
			if eax == 0
			{
				return None
			}

			let ecx = values.ecx;
			Some
			(
				Self
				{
					uncompacted_byte_offset: values.ebx as usize,
					size: values.eax as usize,
					supported_in: unsafe { transmute((ecx & 0b01) as u8) },
					requires_alignment_if_compacted: ecx & 0b10 != 0,
				}
			)
		}
		else
		{
			None
		}
	}
}
