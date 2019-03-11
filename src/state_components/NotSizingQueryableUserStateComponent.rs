// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Represents the state components for `x87` and `SSE`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NotSizingQueryableUserStateComponent
{
	/// The state component used for the x87 FPU execution environment (`x87` state).
	///
	/// See Section 13.5.1 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	x87 = 0,

	/// The state component used for registers used by the streaming SIMD extensions (`SSE` state).
	///
	/// See Section 13.5.2 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	SSE = 1,
}

impl NotSizingQueryableUserStateComponent
{
	/// To an user state component.
	#[inline(always)]
	pub const fn to_user_state_component(self) -> UserStateComponent
	{
		UserStateComponent
		{
			not_sizing_queryable: self
		}
	}

	/// To a state component.
	#[inline(always)]
	pub const fn to_state_component(self) -> StateComponent
	{
		StateComponent
		{
			user: Self::to_user_state_component(self)
		}
	}
}
