// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The `x87` state exists in two, non-contiguous areas ('parts').
///
/// This is part 2.
///
/// It contains the values in the register `ST0`/`MM0` to `ST7`/`MM7` inclusive.
#[derive(Default, Debug, Clone)]
#[repr(C, align(16))]
pub struct X87StatePart2
{
	/// Stored value of floating pointer register `ST0` or legacy MMX register `MM0`.
	///
	/// Part of the `x87` state.
	pub st0_or_mm0: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST1` or legacy MMX register `MM1`.
	///
	/// Part of the `x87` state.
	pub st1_or_mm1: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST2` or legacy MMX register `MM2`.
	///
	/// Part of the `x87` state.
	pub st2_or_mm2: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST3` or legacy MMX register `MM3`.
	///
	/// Part of the `x87` state.
	pub st3_or_mm3: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST4` or legacy MMX register `MM4`.
	///
	/// Part of the `x87` state.
	pub st4_or_mm4: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST5` or legacy MMX register `MM5`.
	///
	/// Part of the `x87` state.
	pub st5_or_mm5: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST6` or legacy MMX register `MM6`.
	///
	/// Part of the `x87` state.
	pub st6_or_mm6: FloatingPointOrMMRegisterValue,

	/// Stored value of floating pointer register `ST7` or legacy MMX register `MM7`.
	///
	/// Part of the `x87` state.
	pub st7_or_mm7: FloatingPointOrMMRegisterValue,
}
