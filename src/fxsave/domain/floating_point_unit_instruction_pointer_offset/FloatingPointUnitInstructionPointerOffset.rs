// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// x87 FPU Instruction Pointer Offset, `FIP`.
///
/// The contents of this field differ depending on the current addressing mode (32-bit, 16-bit, or 64-bit) of the processor when the `FXSAVE` instruction was executed:-
///
/// * 16-bit mode: low 16 bits are instruction pointer (`IP`) offset; high 16 bits are reserved, and, potentially, 16-bit selector (segment)
/// * 32-bit mode: 32-bit instruction pointer (`IP`) offset.
/// * 64-bit mode without `REX.W`, or `REX.W == 0`: 32-bit instruction pointer (`IP`) offset, and, potentially, 16-bit selector (segment).
/// * 64-bit mode with `REX.W`: 64-bit instruction pointer (`IP`) offset.
///
/// See “x87 FPU Instruction and Operand (Data) Pointers” in Chapter 8 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the x87 FPU Instruction Pointer.
#[repr(C)]
pub union FloatingPointUnitInstructionPointerOffset
{
	/// 16-bit mode: low 16 bits are instruction pointer (`IP`) offset; high 16 bits are reserved, and, potentially, 16-bit selector (segment).
	pub mode_16_bit: FloatingPointUnitInstructionPointerOffset16Bit,

	/// 32-bit mode: 32-bit instruction pointer (`IP`) offset, and, potentially, 16-bit selector (segment).
	pub mode_32_bit: FloatingPointUnitInstructionPointerOffset32Bit,

	///  64-bit mode without `REX.W` or `REX.W == 0`: 32-bit instruction pointer (`IP`) offset, and, potentially, 16-bit selector (segment).
	pub mode_64_bit_without_rex_w: FloatingPointUnitInstructionPointerOffset32Bit,

	/// 64-bit mode with `REX.W`: 64-bit instruction pointer (`IP`) offset.
	pub mode_64_bit_with_rex_w: FloatingPointUnitInstructionPointerOffset64BitWithRexW,
}

impl Default for FloatingPointUnitInstructionPointerOffset
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for FloatingPointUnitInstructionPointerOffset
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		unsafe { self.mode_64_bit_with_rex_w.fmt(f) }
	}
}

impl Clone for FloatingPointUnitInstructionPointerOffset
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			mode_64_bit_with_rex_w: unsafe { self.mode_64_bit_with_rex_w }
		}
	}
}
