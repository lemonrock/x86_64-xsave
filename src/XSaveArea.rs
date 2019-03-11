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
	#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))]
	#[inline(always)]
	pub fn save(mut allocator: Allocator, extended_state_information: &ExtendedStateInformation, save_mask: StateComponentBitmap) -> Result<Self, AllocErr>
	{
		let xsave_area_size_supported_features = extended_state_information.xsave_area_size_supported_features;

		let layout = Self::layout(xsave_area_size_enabled_features);
		let pointer = unsafe { NonNull::new_unchecked(allocator.alloc(layout)?.as_ptr() as *mut XSaveAreaLayout) };

		unsafe { _xsave64(pointer.as_ptr() as *mut u8, save_mask.0) };

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
