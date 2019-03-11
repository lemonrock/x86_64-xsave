// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// x87 FPU Data Pointer Offset, `FDP`.
///
/// The contents of this field differ depending on the current addressing mode (32-bit, 16-bit, or 64-bit) of the processor when the `FXSAVE` instruction was executed:-
///
/// * 16-bit mode: low 16 bits are instruction data (`DP`) offset; high 16 bits are reserved, and, potentially, 16-bit selector (segment)
/// * 32-bit mode: 32-bit instruction data (`DP`) offset.
/// * 64-bit mode without `REX.W`, or `REX.W == 0`: 32-bit instruction data (`DP`) offset, and, potentially, 16-bit selector (segment).
/// * 64-bit mode with `REX.W`: 64-bit instruction data (`DP`) offset.
///
/// See “x87 FPU Instruction and Operand (Data) Pointers” in Chapter 8 of the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for a description of the x87 FPU Data Pointer.
#[repr(C)]
pub union FloatingPointUnitDataPointerOffset
{
	/// 16-bit mode: low 16 bits are data pointer (`DP`) offset; high 16 bits are reserved, and, potentially, 16-bit selector (segment).
	pub mode_16_bit: FloatingPointUnitDataPointerOffset16Bit,

	/// 32-bit mode: 32-bit data pointer (`DP`) offset, and, potentially, 16-bit selector (segment).
	pub mode_32_bit: FloatingPointUnitDataPointerOffset32Bit,

	///  64-bit mode without `REX.W` or `REX.W == 0`: 32-bit data pointer (`DP`) offset, and, potentially, 16-bit selector (segment).
	pub mode_64_bit_without_rex_w: FloatingPointUnitDataPointerOffset32Bit,

	/// 64-bit mode with `REX.W`: 64-bit data pointer (`DP`) offset.
	pub mode_64_bit_with_rex_w: FloatingPointUnitDataPointerOffset64BitWithRexW,
}

impl Default for FloatingPointUnitDataPointerOffset
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for FloatingPointUnitDataPointerOffset
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		unsafe { self.mode_64_bit_with_rex_w.fmt(f) }
	}
}

impl Clone for FloatingPointUnitDataPointerOffset
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
