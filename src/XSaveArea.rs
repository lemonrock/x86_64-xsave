// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// A `XSAVE` area.
#[derive(Debug, Clone)]
pub struct XSaveArea<Allocator: Alloc>
{
	allocator: Allocator,
	pointer: NonNull<XSaveAreaLayout>,
	xsave_area_size_supported_features: usize,
}

impl<Allocator: Alloc> Deref for XSaveArea<Allocator>
{
	type Target = XSaveAreaLayout;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { & * self.pointer.as_ptr() }
	}
}

impl<Allocator: Alloc> DerefMut for XSaveArea<Allocator>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.pointer.as_ptr() }
	}
}

impl<Allocator: Alloc> Drop for XSaveArea<Allocator>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = unsafe { NonNull::new_unchecked(self.pointer.as_ptr() as *mut XSaveAreaLayout as *mut u8) };
		let layout = Self::layout(self.xsave_area_size_supported_features);
		unsafe { self.allocator.dealloc(pointer, layout) }
	}
}

impl<Allocator: Alloc> XSaveArea<Allocator>
{
	/// Saves a `XSAVE` area into newly allocated memory.
	#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))]
	#[inline(always)]
	pub fn save(mut allocator: Allocator, extended_state_information: &ExtendedStateInformation, save_mask: StateComponentBitmap) -> Result<Self, AllocErr>
	{
		Self::save_internal(allocator, extended_state_information, |pointer, save_mask| unsafe { _xsave64(pointer, save_mask) })
	}

	/// Saves a `XSAVE` area, compacted, into newly allocated memory.
	#[cfg(all(target_arch = "x86_64", target_feature = "xsave", target_feature = "xsavec"))]
	#[inline(always)]
	pub fn save_compacted(mut allocator: Allocator, extended_state_information: &ExtendedStateInformation, save_mask: StateComponentBitmap) -> Result<Self, AllocErr>
	{
		Self::save_internal(allocator, extended_state_information, |pointer, save_mask| unsafe { _xsavec64(pointer, save_mask) })
	}

	/// Saves a `XSAVE` area, using options in `XCR0`, into newly allocated memory.
	#[cfg(all(target_arch = "x86_64", target_feature = "xsave", target_feature = "xsaveopt"))]
	#[inline(always)]
	pub fn save_using_xcr0_options(mut allocator: Allocator, extended_state_information: &ExtendedStateInformation, save_mask: StateComponentBitmap) -> Result<Self, AllocErr>
	{
		Self::save_internal(allocator, extended_state_information, |pointer, save_mask| unsafe { _xsaveopt64(pointer, save_mask) })
	}

	#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))]
	#[inline(always)]
	fn save_internal(mut allocator: Allocator, extended_state_information: &ExtendedStateInformation, save_mask: StateComponentBitmap, intrinsic_callback: impl Fn(*const u8, u64)) -> Result<Self, AllocErr>
	{
		let xsave_area_size_supported_features = extended_state_information.xsave_area_size_supported_features;

		let layout = Self::layout(xsave_area_size_enabled_features);
		let pointer = unsafe { NonNull::new_unchecked(allocator.alloc(layout)?.as_ptr() as *mut XSaveAreaLayout) };

		intrinsic_callback(pointer.as_ptr() as *mut u8, save_mask.0);

		Self
		{
			allocator,
			pointer,
			xsave_area_size_supported_features,
		}
	}

	/// Restores this `XSAVE` area.
	#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))]
	#[inline(always)]
	pub fn restore(&self)
	{
		unsafe { _xrstor64(self.pointer.as_ptr() as *const XSaveAreaLayout as *const u8) }
	}

	fn layout(xsave_area_size_enabled_features: usize) -> Layout
	{
		unsafe { Layout::from_size_align_unchecked(xsave_area_size_enabled_features, 64) }
	}
}
