// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// A floating point `STj` or legacy MMX `MMj` register value.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FloatingPointOrMMRegisterValue
{
	register_value: [u8; 10],

	reserved: [u8; 6],
}

impl FloatingPointOrMMRegisterValue
{
	/// MM (MMX) value (little endian).
	#[inline(always)]
	pub fn mm_value(&self) -> &[u8; 8]
	{
		array_ref!(self.register_value, 2, 8)
	}

	/// MM (MMX) value (little endian).
	#[inline(always)]
	pub fn mm_value_mut(&mut self) -> &mut [u8; 8]
	{
		array_mut_ref!(self.register_value, 2, 8)
	}

	/// Floating point value (little endian).
	///
	/// x86 extended precision format (80-bit).
	///
	/// * 1-bit for sign (bit 79); if set, the value is negative.
	/// * 15-bits for exponent (bits 78 to 64).
	/// * 64-bits for the significand (bits 63 to 0), of which 1-bit (bit 63) in the 'integer' part.
	#[inline(always)]
	pub fn floating_point_value(&self) -> &[u8; 10]
	{
		&self.register_value
	}

	/// Floating point value (little endian).
	///
	/// x86 extended precision format (80-bit).
	///
	/// * 1-bit for sign (bit 79); if set, the value is negative.
	/// * 15-bits for exponent (bits 78 to 64).
	/// * 64-bits for the significand (bits 63 to 0), of which 1-bit (bit 63) in the 'integer' part.
	#[inline(always)]
	pub fn floating_point_value_mut(&mut self) -> &mut [u8; 10]
	{
		&mut self.register_value
	}
}
