// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// In the Intel 64 and IA-32 architectures, the rounding mode is controlled by a 2-bit rounding-control (`RC`) field.
///
/// Although these two RC fields perform the same function, they control rounding for different execution environments within the processor.
/// The `RC` field in the x87 FPU control register controls rounding for computations performed with the x87 FPU instructions; the `RC` field in the `MXCSR` register controls rounding for SIMD floating-point computations performed with the `SSE`/`SSE2` instructions.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum RoundingControl
{
	/// Round to nearest (even).
	///
	/// Rounded result is the closest to the infinitely precise result.
	///
	/// If two values are equally close, the result is the even value (that is, the one with the least-significant bit of zero).
	/// Default.
	RoundToNearest = 0b00,

	/// Round down (toward −∞).
	///
	/// Rounded result is closest to but no greater than the infinitely precise result.
	RoundDownTowardNegativeInfinity = 0b01,

	/// Round up (toward +∞)
	///
	///Rounded result is closest to but no less than the infinitely precise result.
	RoundUpTowardPositiveInfinity = 0b10,

	/// Round toward zero (Truncate).
	///
	/// Rounded result is closest to but no greater in absolute value than the infinitely precise result.
	RoundTowardZero = 0b11,
}
