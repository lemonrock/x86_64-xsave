// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Layout of a `XSAVE` area.
#[repr(C)]
pub struct XSaveAreaLayout
{
	/// Legacy region.
	///
	/// The legacy region of a `XSAVE` area comprises the 512 bytes starting at the area’s base address.
	/// It is used to manage the state components for x87 state and SSE state.
	/// The legacy region is described in more detail in Section 13.4.1 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	legacy_region: FXSaveArea,

	/// `XSAVE` header.
	///
	/// The `XSAVE` header of a `XSAVE` area comprises the 64 bytes starting at an offset of 512 bytes from the area’s base address.
	/// The `XSAVE` header is described in more detail in Section 13.4.2 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
	xsave_header: XSaveHeader,

	/// Extended region.
	///
	/// The size of the extended region is determined by which state components the processor supports and which bits have been set in `XCR0` (user mode) and `IA32_XSS` `MSR` (supervisor mode) (see Section 13.3 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture)).
	extended_region: XSaveExtendedRegion,
}

impl XSaveAreaLayout
{
	/// `x87` state if enabled.
	///
	/// The `x87` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn x87_state(&self) -> Option<(&X87StatePart1, &X87StatePart2)>
	{
		if self.is_missing(StateComponent::x87)
		{
			None
		}
		else
		{
			Some(self.legacy_region.x87_state())
		}
	}

	/// `SSE` state if enabled.
	///
	/// The `SSE` state is in two non-contiguous areas ('parts').
	#[inline(always)]
	pub fn sse_state(&self) -> Option<(&SseStatePart1, &SseStatePart2)>
	{
		if self.is_missing(StateComponent::SSE)
		{
			None
		}
		else
		{
			Some(self.legacy_region.sse_state())
		}
	}

	/// `AVX` user state if enabled.
	#[inline(always)]
	pub fn avx_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::AVX)
	}

	/// `MPX` `BNDREGS` user state if enabled.
	#[inline(always)]
	pub fn mpx_bdnregs_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::BNDREGS_MPX)
	}

	/// `MPX` `BNDSCR` user state if enabled.
	#[inline(always)]
	pub fn mpx_bndcsr_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::BNDCSR_MPX)
	}

	/// `AVX-512` `opmask` user state if enabled.
	#[inline(always)]
	pub fn avx_512_opmask_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::opmask_AVX_512)
	}

	/// `AVX-512` `ZMM_Hi256` user state if enabled.
	#[inline(always)]
	pub fn avx_512_zmm_hi256_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::ZMM_Hi256_AVX_512)
	}

	/// `AVX-512` `Hi16_ZMM` user state if enabled.
	#[inline(always)]
	pub fn avx_512_hi16_zmm_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::Hi16_ZMM_AVX_512)
	}

	/// `PKRU` user state if enabled.
	#[inline(always)]
	pub fn pkru_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::PKRU)
	}

	/// `PT` supervisor state if enabled.
	#[inline(always)]
	pub fn pt_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::PT)
	}

	/// `HDC` supervisor state if enabled.
	#[inline(always)]
	pub fn hdc_state(&self) -> Option<&[u8]>
	{
		self.extended_state(StateComponent::HDC)
	}

	#[inline(always)]
	fn extended_state(&self, state_component: StateComponent) -> Option<&[u8]>
	{
		if self.is_missing(state_component)
		{
			None
		}
		else
		{
			let sizing = match state_component.sizing()
			{
				None => return None,
				Some(sizing) => sizing,
			};

			// Based on Section 13.4.3 (Extended Region of an XSAVE Area) of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
			let offset = if self.is_extended_region_uncompacted()
			{
				sizing.uncompacted_byte_offset
			}
			else
			{
				const BaseOffset: usize = 576;

				// "If XCOMP_BV[j] = 0 for every j, 2 ≤ j < i, locationI is 576".
				let mask = (1 << (state_component.bit() + 1)) & 0b00;
				let offset = if self.xsave_header.XCOMP_BV.bits_are_zero(mask)
				{
					BaseOffset
				}
				else
				{
					let mut offset = BaseOffset;
					for bit in 2 .. (state_component.bit() - 1)
					{
						if self.xsave_header.XCOMP_BV.bit_set(bit)
						{
							if let Some(sizing) = StateComponentSizing::sizing(bit)
							{
								offset += sizing.size;
								if sizing.requires_alignment_if_compacted
								{
									#[inline(always)]
									const fn round_up_64(value: usize) -> usize
									{
										(value + (64 - 1)) & !(64 -1)
									}

									offset = round_up_64(offset)
								}
							}
						}
					}
					offset
				};

				offset
			};

			let starts_at_pointer = (self as *const Self as usize) + offset;
			Some(unsafe { from_raw_parts(starts_at_pointer as *const u8, sizing.size) })
		}
	}

	#[inline(always)]
	fn is_extended_region_uncompacted(&self) -> bool
	{
		!self.xsave_header.is_extended_region_compacted()
	}

	#[inline(always)]
	fn is_missing(&self, state_component: StateComponent) -> bool
	{
		!self.is_present(state_component)
	}

	#[inline(always)]
	fn is_present(&self, state_component: StateComponent) -> bool
	{
		self.xsave_header.is_present(state_component)
	}
}
