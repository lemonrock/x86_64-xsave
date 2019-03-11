// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The `SSE` state exists in two, non-contiguous areas ('parts').
///
/// This is part 1.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct SseStatePart1
{
	/// Value of the `MXCSR` register.
	///
	/// Part of the `SSE` state.
	pub mxcsr_register_value: MxcsrRegisterValue,

	/// `MXCSR` register mask.
	///
	/// Part of the `SSE` state.
	///
	/// (Not restored (read) when restoring processor state).
	pub mxcsr_register_mask: u32,
}