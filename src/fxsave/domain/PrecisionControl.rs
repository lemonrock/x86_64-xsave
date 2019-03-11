// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Precision control determines the precision (64, 53, or 24 bits) of floating-point calculations made by the x87 FPU.
/// The default precision is double extended precision, which uses the full 64-bit significand available with the double extended-precision floating-point format of the x87 FPU data registers.
/// This setting is best suited for most applications, because it allows applications to take full advantage of the maximum precision available with the x87 FPU data registers.
///
/// The double precision and single precision settings reduce the size of the significand to 53 bits and 24 bits, respectively.
/// These settings are provided to support IEEE Standard 754 and to provide compatibility with the specifications of certain existing programming languages.
/// Using these settings nullifies the advantages of the double extended-precision floating-point format's 64-bit significand length.
/// When reduced precision is specified, the rounding of the significand value clears the unused bits on the right to zeros.
///
/// Returns a 2-bit (`u2`) value:-
///
/// * `0b00`: Single Precision (24 bits).
/// * `0b01`: Reserved.
/// * `0b10`: Double Precision (53 bits).
/// * `0b11`: Double Extended Precision (64 bits)
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PrecisionControl
{
	/// Single Precision (24 bits).
	SinglePrecision = 0b00_0000_0000,

	/// Reserved.
	Reserved = 0b01_0000_0000,

	/// Double Precision (53 bits).
	DoublePrecision = 0b10_0000_0000,

	/// Double Extended Precision (64 bits).
	DoubleExtendedPrecision = 0b11_0000_0000,
}
