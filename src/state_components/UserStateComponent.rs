// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// An user state component.
#[derive(Copy, Clone)]
pub union UserStateComponent
{
	/// An user state component whose size can not be queried using `CPUID`.
	pub not_sizing_queryable: NotSizingQueryableUserStateComponent,

	/// An user state component whose size can be queried using `CPUID`.
	pub sizing_queryable: SizingQueryableUserStateComponent,
}

impl UserStateComponent
{
	/// `x87` user state component.
	pub const x87: UserStateComponent = NotSizingQueryableUserStateComponent::to_user_state_component(NotSizingQueryableUserStateComponent::x87);

	/// `SSE` user state component.
	pub const SSE: UserStateComponent = NotSizingQueryableUserStateComponent::to_user_state_component(NotSizingQueryableUserStateComponent::SSE);

	/// `AVX` user state component.
	pub const AVX: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::AVX);

	/// `BNDREGS_MPX` user state component.
	pub const BNDREGS_MPX: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::BNDREGS_MPX);

	/// `BNDCSR_MPX` user state component.
	pub const BNDCSR_MPX: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::BNDCSR_MPX);

	/// `opmask_AVX_512` user state component.
	pub const opmask_AVX_512: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::opmask_AVX_512);

	/// `ZMM_Hi256_AVX_512` user state component.
	pub const ZMM_Hi256_AVX_512: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::ZMM_Hi256_AVX_512);

	/// `Hi16_ZMM_AVX_512` user state component.
	pub const Hi16_ZMM_AVX_512: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::Hi16_ZMM_AVX_512);

	/// `PKRU` user state component.
	pub const PKRU: UserStateComponent = SizingQueryableUserStateComponent::to_user_state_component(SizingQueryableUserStateComponent::PKRU);

	/// To a state component.
	#[inline(always)]
	pub const fn to_state_component(self) -> StateComponent
	{
		StateComponent
		{
			user: self
		}
	}
}
