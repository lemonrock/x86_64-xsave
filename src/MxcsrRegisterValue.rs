// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The 32-bit MXCSR register contains control and status information for `SSE`, `SSE2`, and `SSE3` SIMD floating-point operations.
///
/// This register contains:-
/// * flag and mask bits for SIMD floating-point exceptions;
/// *  rounding control field for SIMD floating-point operations;
/// * flush-to-zero flag that provides a means of controlling underflow conditions on SIMD floating-point operations;
/// * denormals-are-zeros flag that controls how SIMD floating-point instructions handle denormal source operands
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MxcsrRegisterValue(u32);

impl MxcsrRegisterValue
{
	/// Get current value in register.
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
	#[inline(always)]
	pub fn save_current_value_in_register() -> Self
	{
		MxcsrRegisterValue(unsafe { _mm_getcsr() })
	}

	/// Set current value in register.
	///
	/// Only affects the current thread.
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
	#[inline(always)]
	pub fn restore_current_value_in_register(&self)
	{
		unsafe { _mm_setcsr(self.0) }
	}

	/// Update the current value of the MXCSR register.
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
	#[inline(always)]
	pub fn update_from_current_value_in_register(&mut self)
	{
		*self = MxcsrRegisterValue(unsafe { _mm_getcsr() });
	}

	/// Denormals Are Zeros, `DAZ`.
	#[inline(always)]
	pub fn denormals_are_zeros(self) -> bool
	{
		self.bit_set(6)
	}

	/// Flag bits 0 to 5 inclusive.
	///
	/// A 6-bit (`u6`) value.
	#[inline(always)]
	pub fn flag_bits(self) -> u8
	{
		(self.0 & 0b0011_1111) as u8
	}

	/// Invalid Operation exception flag, `IE`.
	#[inline(always)]
	pub fn invalid_operation_flag(self) -> bool
	{
		self.bit_set(0)
	}

	/// Denormal exception flag, `DE`.
	#[inline(always)]
	pub fn denormal_flag(self) -> bool
	{
		self.bit_set(1)
	}

	/// Divide-by-Zero exception flag, `ZE`.
	#[inline(always)]
	pub fn divide_by_zero_flag(self) -> bool
	{
		self.bit_set(2)
	}

	/// Overflow exception flag, `OE`.
	#[inline(always)]
	pub fn overflow_flag(self) -> bool
	{
		self.bit_set(3)
	}

	/// Underflow exception flag, `UE`.
	#[inline(always)]
	pub fn underflow_flag(self) -> bool
	{
		self.bit_set(4)
	}

	/// Precision exception flag, `PE`.
	#[inline(always)]
	pub fn precision_flag(self) -> bool
	{
		self.bit_set(5)
	}

	/// Flag bits 0 to 5 inclusive.
	///
	/// A 6-bit (`u6`) value.
	#[inline(always)]
	pub fn mask_bits(self) -> u8
	{
		((self.0 & 0b00001_1111_1000_0000) >> 7) as u8
	}

	/// Invalid Operation exception mask bit, `IM`.
	#[inline(always)]
	pub fn invalid_operation_mask_bit(self) -> bool
	{
		self.bit_set(7)
	}

	/// Denormal exception mask bit, `DM`.
	#[inline(always)]
	pub fn denormal_mask_bit(self) -> bool
	{
		self.bit_set(8)
	}

	/// Divide-by-Zero exception mask bit, `ZM`.
	#[inline(always)]
	pub fn divide_by_zero_mask_bit(self) -> bool
	{
		self.bit_set(9)
	}

	/// Overflow exception mask bit, `OM`.
	#[inline(always)]
	pub fn overflow_mask_bit(self) -> bool
	{
		self.bit_set(10)
	}

	/// Underflow exception mask bit, `UM`.
	#[inline(always)]
	pub fn underflow_mask_bit(self) -> bool
	{
		self.bit_set(11)
	}

	/// Precision exception mask bit, `PM`.
	#[inline(always)]
	pub fn precision_mask_bit(self) -> bool
	{
		self.bit_set(12)
	}

	/// Rounding control, `RC`.
	#[inline(always)]
	pub fn rounding_control(self) -> RoundingControl
	{
		unsafe { transmute((self.0 >> 13) as u8) }
	}

	/// Precision exception mask bit, `PM`.
	#[inline(always)]
	pub fn flush_to_zero(self) -> bool
	{
		self.bit_set(15)
	}

	#[inline(always)]
	fn bit_set(self, bit: u32) -> bool
	{
		self.0 & (1 << bit) != 0
	}
}
