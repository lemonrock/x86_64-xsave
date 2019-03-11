// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The x87 FPU stores in the 11-bit x87 FPU opcode register (FOP) the opcode of the last x87 non-control instruction executed that incurred an unmasked x87 exception.
/// This information provides state information for exception handlers.
///
/// Only the first and second opcode bytes (after all prefixes) are stored in the x87 FPU opcode register.
///
/// Since the upper 5 bits of the first opcode byte are the same for all floating-point opcodes (0b11011), only the lower 3 bits of this byte are stored in the opcode register.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FloatingPointUnitOpcode(u16);

impl FloatingPointUnitOpcode
{
	/// The first opcode byte.
	#[inline(always)]
	pub fn first_opcode_byte(self) -> u8
	{
		const upper_5_bit_of_first_opcode: u8 = 0b11011 << 3;
		let lower_3_bits_of_first_opcode = (self.0 & 0b0000_0111_0000_0000 >> 8) as u8;

		upper_5_bit_of_first_opcode | lower_3_bits_of_first_opcode
	}

	/// The second opcode byte.
	#[inline(always)]
	pub fn second_opcode_byte(self) -> u8
	{
		(self.0 & 0b1111_1111) as u8
	}
}
