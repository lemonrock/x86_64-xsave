// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The `x87` state exists in two, non-contiguous areas ('parts').
///
/// This is part 1.
#[derive(Default, Debug, Clone)]
#[repr(C, align(16))]
pub struct X87StatePart1
{
	/// x87 FPU Control Word, `FCW`.
	///
	/// Part of the `x87` state.
	pub fpu_control_word: X87FloatingPointUnitControlWord,

	/// x87 FPU Status Word, `FSW`.
	///
	/// Part of the `x87` state.
	///
	/// See Figure 8-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the layout of the x87 FPU Status Word.
	pub fpu_status_word: X87FloatingPointUnitStatusWord,

	/// Abridged FPU Tag Word, `FTW`.
	///
	/// Part of the `x87` state.
	///
	/// See the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, Section 8.1.7 for a description of the unabridged FPU Tag Word.
	pub abridged_fpu_tag_word: AbridgedFpuTagWord,

	reserved0: u8,

	/// x87 FPU Opcode, `FOP`.
	///
	/// Part of the `x87` state.
	pub fpu_opcode: FloatingPointUnitOpcode,

	/// x87 FPU Instruction Pointer Offset, `FIP`.
	///
	/// Part of the `x87` state.
	pub fpu_instruction_pointer_offset: FloatingPointUnitInstructionPointerOffset,

	/// x87 FPU Data Pointer Offset, `FDP`.
	///
	/// Part of the `x87` state.
	pub fpu_data_pointer_offset: FloatingPointUnitDataPointerOffset,
}
