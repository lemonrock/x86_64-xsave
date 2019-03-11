// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// 16-bit mode FPU instruction pointer (`FIP`).
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct FloatingPointUnitDataPointerOffset16Bit
{
	/// x87 FPU Data Pointer Offset, `FDP`.
	pub data_pointer_offset: u16,

	reserved0: u16,

	/// x87 FPU Instruction Pointer Selector, `FDS` or `FPU DS`.
	///
	/// Value of a segmentation register.
	///
	/// If `CPUID.(EAX=07H,ECX=0H):EBX[bit 13] = 1`, the processor deprecates `FCS` and `FDS`, and this field is saved as 0x0000.
	pub data_pointer_selector: u16,

	reserved1: u16,
}

impl PartialEq for FloatingPointUnitDataPointerOffset16Bit
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.data_pointer_selector == other.data_pointer_selector && self.data_pointer_offset == other.data_pointer_offset
	}
}

impl Eq for FloatingPointUnitDataPointerOffset16Bit
{
}

impl PartialOrd for FloatingPointUnitDataPointerOffset16Bit
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for FloatingPointUnitDataPointerOffset16Bit
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.data_pointer_selector.cmp(&other.data_pointer_selector).then_with(|| self.data_pointer_offset.cmp(&other.data_pointer_offset))
	}
}

impl Hash for FloatingPointUnitDataPointerOffset16Bit
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.data_pointer_selector.hash(state);
		self.data_pointer_offset.hash(state);
	}
}
