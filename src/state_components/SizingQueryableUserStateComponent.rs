// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// An user state component whose size can be queried.
///
/// Support can be checked in the `XCR0` register.
///
/// To access the `XCR0` register use the struct `StateComponentBitmap`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SizingQueryableUserStateComponent
{
	/// The state component used for the additional register state used by the Intel® Advanced Vector Extensions (`AVX` state).
	AVX = 2,

	/// The state component used for the additional register state used by Intel® Memory Protection Extensions (`MPX` state) for the four 128-bit bounds registers `BND0`-`BND3` (`BNDREGS` state).
	BNDREGS_MPX = 3,

	/// The state component used for the additional register state used by Intel® Memory Protection Extensions (`MPX` state) for the 64-bit user-mode `MPX` configuration register `BNDCFGU` and the 64-bit `MPX` status register `BNDSTATUS (`BNDSCR` state).
	BNDCSR_MPX = 4,

	/// The state component used for the the additional register state used by Intel® Advanced Vector Extensions 512 (`AVX-512` state) for the eight 64-bit `opmask` registers `k0`–`k7` (`opmask` state).
	opmask_AVX_512 = 5,

	/// The state component used for the the additional register state used by Intel® Advanced Vector Extensions 512 (`AVX-512` state) the upper 256 bits of the registers `ZMM0`–`ZMM15` (`ZMM_Hi256` state).
	ZMM_Hi256_AVX_512 = 6,

	/// The state component used for the the additional register state used by Intel® Advanced Vector Extensions 512 (`AVX-512` state) for the sixteen 512-bit registers `ZMM16`–`ZMM31` (`Hi16_ZMM` state).
	Hi16_ZMM_AVX_512 = 7,

	/// The state component used for the protection-key feature’s register `PKRU` (`PKRU` state).
	///
	/// See Section 13.5.7 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	PKRU = 9,
}

impl SizingQueryableUserStateComponent
{
	/// To an user state component.
	#[inline(always)]
	pub const fn to_user_state_component(self) -> UserStateComponent
	{
		UserStateComponent
		{
			sizing_queryable: self
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

	/// Query size using `CPUID` of this user state component.
	///
	/// Returns `None` if not supported.
	#[inline(always)]
	pub fn sizing(self) -> Option<StateComponentSizing>
	{
		StateComponentSizing::sizing(self as u8)
	}
}
