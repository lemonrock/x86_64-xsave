// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// See Figure 8-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the layout of the x87 FPU Control Word.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct X87FloatingPointUnitControlWord(u16);

impl X87FloatingPointUnitControlWord
{
	/// Reads the control word after raising any pending unmasked floating point exceptions.
	///
	/// Uses the `FSTCW` instruction; see <https://github.com/HJLebbink/asm-dude/wiki/FSTCW_FNSTCW>.
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	#[inline(always)]
	pub fn save_after_raising_any_pending_unmasked_floating_point_exceptions() -> Self
	{
		let mut control_word: u16 = unsafe { uninitialized() };
		let control_word_pointer = &mut control_word;

		unsafe
		{
			asm!
			(
				"fstcw $0"
				:
					// Output constraints.
					"=*m"(control_word_pointer)
				:
					// Input constraints.
				:
					// Clobbers.
				:
					// Options.
					"volatile"
			);
		}

		Self(control_word)
	}

	/// Reads the control word.
	///
	/// Uses the `FNSTCW` instruction; see <https://github.com/HJLebbink/asm-dude/wiki/FSTCW_FNSTCW>.
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	#[inline(always)]
	pub fn save() -> Self
	{
		let mut control_word: u16 = unsafe { uninitialized() };
		let control_word_pointer = &mut control_word;

		unsafe
		{
			asm!
			(
				"fnstcw $0"
				:
					// Output constraints.
					"=*m"(control_word_pointer)
				:
					// Input constraints.
				:
					// Clobbers.
				:
					// Options.
					"volatile"
			);
		}

		Self(control_word)
	}

	/// Writes the control word.
	///
	/// Uses the `FLDCW` instruction; see <https://github.com/HJLebbink/asm-dude/wiki/FLDCW>.
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	#[inline(always)]
	pub fn restore(mut self)
	{
		let control_word_pointer = &mut self.0;
		unsafe
		{
			asm!
			(
				"fnstcw $0"
				:
					// Output constraints.
				:
					// Input constraints.
					"*m"(control_word_pointer)
				:
					// Clobbers.
				:
					// Options.
					"volatile"
			);
		}
	}

	/// Invalid Operation, `IM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_invalid_operation(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0001 != 0
	}

	/// Denormal Operation, `DM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_denormal_operation(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0010 != 0
	}

	/// Zero Divide, `ZM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_zero_divide(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0100 != 0
	}

	/// Overflow, `OM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_overflow(self) -> bool
	{
		self.0 & 0b0000_0000_0000_1000 != 0
	}

	/// Underflow, `UM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_underflow(self) -> bool
	{
		self.0 & 0b0000_0000_0001_0000 != 0
	}

	/// Precision, `PM`.
	///
	/// One of the 6 exception flag mask bits.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception is blocked from being generated.
	#[inline(always)]
	pub fn exception_flag_mask_bit_precision(self) -> bool
	{
		self.0 & 0b0000_0000_0010_0000 != 0
	}

	/// Precision Control, `PC`.
	#[inline(always)]
	pub fn precision_control(self) -> PrecisionControl
	{
		unsafe { transmute(self.0 & 0b0000_0011_0000_0000) }
	}

	/// Rounding Control, `RC`.
	///
	/// A 2-bit (`u2`) value.
	#[inline(always)]
	pub fn rounding_control(self) -> RoundingControl
	{
		unsafe { transmute((self.0 & 0b0000_1100_0000_0000 >> 10) as u8) }
	}

	/// Infinity Control, `X`, flag.
	///
	/// This is provided for compatibility with the Intel 287 Math Coprocessor; it is not meaningful for later version x87 FPU coprocessors or IA-32 processors.
	#[inline(always)]
	pub fn infinity_control(self) -> bool
	{
		self.0 & 0b0001_0000_0000_0000 != 0
	}
}
