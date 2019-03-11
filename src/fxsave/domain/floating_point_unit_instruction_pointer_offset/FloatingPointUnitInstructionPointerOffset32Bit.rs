// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// 32-bit mode or 64-bit mode without `REX.W` or `REX.W == 0` FPU instruction pointer (`FIP`).
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct FloatingPointUnitInstructionPointerOffset32Bit
{
	/// x87 FPU Instruction Pointer Offset, `FIP`.
	pub instruction_pointer_offset: u32,

	/// x87 FPU Instruction Pointer Selector, `FCS` or `FPU CS`.
	///
	/// Value of a segmentation register.
	///
	/// If `CPUID.(EAX=07H,ECX=0H):EBX[bit 13] = 1`, the processor deprecates `FCS` and `FDS`, and this field is saved as 0x0000.
	pub instruction_pointer_selector: u16,

	reserved: u16,
}

impl PartialEq for FloatingPointUnitInstructionPointerOffset32Bit
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.instruction_pointer_selector == other.instruction_pointer_selector && self.instruction_pointer_offset == other.instruction_pointer_offset
	}
}

impl Eq for FloatingPointUnitInstructionPointerOffset32Bit
{
}

impl PartialOrd for FloatingPointUnitInstructionPointerOffset32Bit
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for FloatingPointUnitInstructionPointerOffset32Bit
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.instruction_pointer_selector.cmp(&other.instruction_pointer_selector).then_with(|| self.instruction_pointer_offset.cmp(&other.instruction_pointer_offset))
	}
}

impl Hash for FloatingPointUnitInstructionPointerOffset32Bit
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.instruction_pointer_selector.hash(state);
		self.instruction_pointer_offset.hash(state)
	}
}
