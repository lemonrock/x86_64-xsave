// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The tag information stored in this structure is abridged:-
///
/// * For each `j`, `0 ≤ j ≤ 7`, `FXSAVE` does as follows:-
///   * Saves a `0` into bit `j` if x87 FPU data register `STj` has an empty tag;
///   * Orherwise, saves a `1` into bit `j`.
/// * For each `j`, `0 ≤ j ≤ 7`, `FXRSTOR` establishes the tag value for x87 FPU data register `STj` as follows:-
///   * If bit `j` is `0`, the tag for `STj` in the tag register for that data register is marked empty
///   * Otherwise, the x87 FPU sets the tag for `STj` based on the value being loaded into that register (see below).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AbridgedFpuTagWord(u8);

impl AbridgedFpuTagWord
{
	/// FPU data register `STj` has an empty tag.
	///
	/// `j` must be between 0 and 7 inclusive.
	#[inline(always)]
	pub fn fpu_data_register_STj_has_an_empty_tag(self, j: u8) -> bool
	{
		debug_assert!(j < 8, "j `{}` is greater than 7", j);

		self.0 & (1 << j) != 0
	}

	/// FPU data register `ST0` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST0_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(0)
	}

	/// FPU data register `ST1` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST1_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(1)
	}

	/// FPU data register `ST2` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST2_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(2)
	}

	/// FPU data register `ST3` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST3_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(3)
	}

	/// FPU data register `ST4` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST4_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(4)
	}

	/// FPU data register `ST5` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST5_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(5)
	}

	/// FPU data register `ST6` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST6_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(6)
	}

	/// FPU data register `ST7` has an empty tag.
	#[inline(always)]
	pub fn fpu_data_register_ST7_has_an_empty_tag(self) -> bool
	{
		self.fpu_data_register_STj_has_an_empty_tag(7)
	}
}
