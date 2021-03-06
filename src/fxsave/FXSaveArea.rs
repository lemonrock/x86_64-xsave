// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Used to save and restore the state of the FPU.
/// Must be 16-byte aligned (enforced).
///
/// Always 512 bytes in size.
///
/// Documented in Table 10-2 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1 and repeated in the documentation of the `XSAVE` area in Section 13.4.
///
/// The legacy instructions `FSAVE` and `FNSAVE` (save FPU state) and `FRSTOR` (restore FPU state) also work with with this layout.
/// The legacy instructions `FSTENV` and `FNSTENV` (save FPU environment) and `FLDENV` (restore FPU environment) also work with the layout in `x87_state_part_1`.
#[derive(Default, Debug, Clone)]
#[repr(C, align(16))]
pub struct FXSaveArea
{
	/// The `x87` state exists in two, non-contiguous areas ('parts').
	///
	/// This is part 1.
	pub x87_state_part_1: X87StatePart1,

	/// The `SSE` state exists in two, non-contiguous areas ('parts').
	///
	/// This is part 1.
	pub sse_state_part_1: SseStatePart1,

	/// The `x87` state exists in two, non-contiguous areas ('parts').
	///
	/// This is part 2.
	pub x87_state_part_2: X87StatePart2,

	/// The `SSE` state exists in two, non-contiguous areas ('parts').
	///
	/// This is part 2.
	pub sse_state_part_2: SseStatePart2,

	reserved1: [u64; 6],

	/// These bytes are unused are never written to or read from.
	///
	/// (Not restored (read) when restoring processor state).
	pub unused: [u64; 6],
}

impl FXSaveArea
{
	/// Saves this `FXSAVE` area.
	///
	/// Used to save and restore the state of the FPU (x87) and MMX (SSE).
	///
	/// Unlike the legacy instructions `FSAVE` and `FNSAVE`, this does not change the state of the FPU (x87) and MMX (SSE) units, registers and associated data.
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "fxsr"))]
	#[inline(always)]
	pub fn save() -> Self
	{
		let mut this = unsafe { uninitialized() };
		let pointer = &mut this as *mut Self as *mut u8;
		unsafe
		{
			#[cfg(target_arch = "x86")] _fxsave(pointer);
			#[cfg(target_arch = "x86_64")] _fxsave64(pointer);
		};
		this
	}

	/// Restores this `FXSAVE` area.
	///
	/// If any reserved bits are set in `sse_state().0.mxcsr_register_value`, then a restore of this value will cause a general-protection fault (`#GP`).
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "fxsr"))]
	#[inline(always)]
	pub fn restore(&self)
	{
		let pointer = self as *const Self as *const u8;
		unsafe
		{
			#[cfg(target_arch = "x86")] _fxrstor(pointer);
			#[cfg(target_arch = "x86_64")] _fxrstor64(pointer);
		}
	}

	/// `x87` state.
	///
	/// The `x87` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn x87_state(&self) -> (&X87StatePart1, &X87StatePart2)
	{
		(&self.x87_state_part_1, &self.x87_state_part_2)
	}

	/// `x87` state.
	///
	/// The `x87` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn x87_state_mut(&mut self) -> (&mut X87StatePart1, &mut X87StatePart2)
	{
		(&mut self.x87_state_part_1, &mut self.x87_state_part_2)
	}

	/// `SSE` state.
	///
	/// The `SSE` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn sse_state(&self) -> (&SseStatePart1, &SseStatePart2)
	{
		(&self.sse_state_part_1, &self.sse_state_part_2)
	}

	/// `SSE` state.
	///
	/// The `SSE` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn sse_state_mut(&mut self) -> (&mut SseStatePart1, &mut SseStatePart2)
	{
		(&mut self.sse_state_part_1, &mut self.sse_state_part_2)
	}
}
