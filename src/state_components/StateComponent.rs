// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// An state component.
#[derive(Copy, Clone)]
pub union StateComponent
{
	/// An user state component.
	pub user: UserStateComponent,

	/// A supervisor state component.
	pub supervisor: SupervisorStateComponent,
}

impl StateComponent
{
	/// `x87` user state component.
	pub const x87: StateComponent = UserStateComponent::x87.to_state_component();

	/// `SSE` user state component.
	pub const SSE: StateComponent = UserStateComponent::SSE.to_state_component();

	/// `AVX` user state component.
	pub const AVX: StateComponent = UserStateComponent::AVX.to_state_component();

	/// `BNDREGS_MPX` user state component.
	pub const BNDREGS_MPX: StateComponent = UserStateComponent::BNDREGS_MPX.to_state_component();

	/// `BNDCSR_MPX` user state component.
	pub const BNDCSR_MPX: StateComponent = UserStateComponent::BNDCSR_MPX.to_state_component();

	/// `opmask_AVX_512` user state component.
	pub const opmask_AVX_512: StateComponent = UserStateComponent::opmask_AVX_512.to_state_component();

	/// `ZMM_Hi256_AVX_512` user state component.
	pub const ZMM_Hi256_AVX_512: StateComponent = UserStateComponent::ZMM_Hi256_AVX_512.to_state_component();

	/// `Hi16_ZMM_AVX_512` user state component.
	pub const Hi16_ZMM_AVX_512: StateComponent = UserStateComponent::Hi16_ZMM_AVX_512.to_state_component();

	/// `PKRU` user state component.
	pub const PKRU: StateComponent = UserStateComponent::PKRU.to_state_component();

	/// `PT` supervisor state component.
	pub const PT: StateComponent = SupervisorStateComponent::PT.to_state_component();

	/// `HDC` supervisor state component.
	pub const HDC: StateComponent = SupervisorStateComponent::HDC.to_state_component();

	/// Convenience function to construct a state component.
	#[inline(always)]
	pub const fn not_sizing_queryable_user_state_component(not_sizing_queryable_user_state_component: NotSizingQueryableUserStateComponent) -> StateComponent
	{
		not_sizing_queryable_user_state_component.to_state_component()
	}

	/// Convenience function to construct a state component.
	#[inline(always)]
	pub const fn sizing_queryable_user_state_component(sizing_queryable_user_state_component: SizingQueryableUserStateComponent) -> StateComponent
	{
		sizing_queryable_user_state_component.to_state_component()
	}

	/// Convenience function to construct a state component.
	#[inline(always)]
	pub const fn sizing_queryable_supervisor_state_component(supervisor_state_component: SupervisorStateComponent) -> StateComponent
	{
		supervisor_state_component.to_state_component()
	}

	/// Convenience function to construct a state component.
	#[inline(always)]
	pub const fn user_state_component(user_state_component: UserStateComponent) -> StateComponent
	{
		user_state_component.to_state_component()
	}

	/// Query size using `CPUID` of the state component for `bit`.
	///
	/// `bit` must be between 2 and 62 inclusive.
	#[inline(always)]
	pub fn sizing(self) -> Option<StateComponentSizing>
	{
		let bit = self.bit();
		if bit >= 2
		{
			StateComponentSizing::sizing(bit)
		}
		else
		{
			None
		}
	}

	/// State component bit value.
	#[inline(always)]
	pub fn bit(self) -> u8
	{
		unsafe { transmute(self) }
	}
}
