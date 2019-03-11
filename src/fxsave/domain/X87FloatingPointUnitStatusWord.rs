// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// See Figure 8-6 in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, for the layout of the x87 FPU Control Word.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct X87FloatingPointUnitStatusWord(u16);

impl X87FloatingPointUnitStatusWord
{
	/// Invalid Operation exception flag, `IE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_invalid_operation(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0001 != 0
	}

	/// Denormalized Operand exception flag, `DE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_denormalized_operand(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0010 != 0
	}

	/// Zero Divide exception flag, `ZE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_zero_divide(self) -> bool
	{
		self.0 & 0b0000_0000_0000_0100 != 0
	}

	/// Overflow exception flag, `OE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_overflow(self) -> bool
	{
		self.0 & 0b0000_0000_0000_1000 != 0
	}

	/// Underflow exception flag, `UE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_underflow(self) -> bool
	{
		self.0 & 0b0000_0000_0001_0000 != 0
	}

	/// Precision exception flag, `EE`.
	///
	/// One of the 6 exception flags that can be controlled by the status word.
	///
	/// When set (`true`), the corresponding x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_flag_precision(self) -> bool
	{
		self.0 & 0b0000_0000_0010_0000 != 0
	}

	/// Stack Fault exception flag, `SF`.
	///
	/// The stack fault flag indicates that stack overflow or stack underflow has occurred with data in the x87 FPU data register stack.
	/// The x87 FPU explicitly sets the `SF` flag when it detects a stack overflow or underflow condition, but it does not explicitly clear the flag when it detects an invalid-arithmetic-operand condition.
	/// When this flag is set, the condition code flag `C1` indicates the nature of the fault: overflow (`C1 = 1`) and underflow (`C1 = 0`).
	///
	/// The `SF` flag is a “sticky” flag, meaning that after it is set, the processor does not clear it until it is explicitly instructed to do so (for example, by an `FINIT`/`FNINIT`, `FCLEX`/`FNCLEX`, or `FSAVE`/`FNSAVE` instruction).
	#[inline(always)]
	pub fn stack_fault(self) -> bool
	{
		self.0 & 0b0000_0000_0100_0000 != 0
	}

	/// Exception Summary Status, `ES`.
	///
	/// When set (`true`), a x87 FPU floating-point exception occurred.
	#[inline(always)]
	pub fn exception_summary_status(self) -> bool
	{
		self.0 & 0b0000_0000_1000_0000 != 0
	}

	/// Condition Code `C0`.
	///
	/// See Table 8-1 (Condition Code Interpretation) in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
	#[inline(always)]
	pub fn condition_code_c0(self) -> bool
	{
		self.0 & 0b0000_0001_0000_0000 != 0
	}

	/// Condition Code `C1`.
	///
	/// See Table 8-1 (Condition Code Interpretation) in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
	#[inline(always)]
	pub fn condition_code_c1(self) -> bool
	{
		self.0 & 0b0000_0010_0000_0000 != 0
	}

	/// Condition Code `C2`.
	///
	/// See Table 8-1 (Condition Code Interpretation) in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
	#[inline(always)]
	pub fn condition_code_c2(self) -> bool
	{
		self.0 & 0b0000_0100_0000_0000 != 0
	}

	/// Condition Code `C3`.
	///
	/// See Table 8-1 (Condition Code Interpretation) in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1.
	#[inline(always)]
	pub fn condition_code_c3(self) -> bool
	{
		self.0 & 0b0100_0000_0000_0000 != 0
	}

	/// Top of Stack Pointer, `TOP`.
	///
	/// A pointer to the x87 FPU data register that is currently at the top of the x87 FPU register stack.
	///
	/// This pointer is a binary value from 0 to 7 (A 3-bit (`u3`) value).
	#[inline(always)]
	pub fn top(self) -> u8
	{
		(self.0 & 0b0011_1000_0000_0000 >> 11) as u8
	}

	/// FPU Busy, `B`, or `B-bit`.
	///
	/// This is for 8087 compatibility only.
	/// It reflects the contents of the `ES` (`self.exception_summary_status()`) flag.
	#[inline(always)]
	pub fn fpu_busy(self) -> bool
	{
		self.0 & 0b1000_0000_0000_0000 != 0
	}
}
