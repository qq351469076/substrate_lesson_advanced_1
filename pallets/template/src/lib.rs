#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	// usually use macro and  Return Result Type
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	// System depend on Type Alice and Data Type
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	// Config Interface
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
		type AssertDepositBase:Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransferred(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		MaxLengthLimit,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			// get sender | AccountId
			let who = ensure_signed(origin)?;

			ensure!((*&claim.len() as u32) <= T::AssertDepositBase::get(), Error::<T>::MaxLengthLimit);

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(&claim, (who.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimCreated(who, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!((*&claim.len() as u32) <= T::AssertDepositBase::get(), Error::<T>::MaxLengthLimit);

			let (sender, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(who == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(who, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			owner: T::AccountId,
			claim: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!((*&claim.len() as u32) <= T::AssertDepositBase::get(), Error::<T>::MaxLengthLimit);

			let (who, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(sender == who, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&claim, (owner.clone(), frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransferred(sender, claim));

			Ok(().into())
		}
	}
}
