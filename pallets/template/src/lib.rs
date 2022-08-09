#![cfg_attr(not(feature = "std"), no_std)]
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
	/// Event emitted when a claim has been created.
	ClaimCreated { who: T::AccountId, claim: T::Hash },
	/// Event emitted when a claim is revoked by the owner.
	ClaimRevoked { who: T::AccountId, claim: T::Hash },
  } 
  #[pallet::error]   // <-- Step 4. code block will replace this.
  pub enum Error<T> {
	// Claim already exists
	AlreadyClaimed,
	// The claim does not exists
	NoSuchClaim,
	// The claim is owned by other account so cannot revoke
	NotClaimOwner
  }
  #[pallet::storage] // <-- Step 5. code block will replace this.
  pub(super) type Claims <T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;
  #[pallet::call]    // <-- Step 6. code block will replace this.
  impl<T: Config> Pallet<T> {
	#[pallet::weight(0)]
	pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
		let sender = ensure_signed(origin)?;

		ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);
		let current_block = <frame_system::Pallet<T>>::block_number();

		Claims::<T>::insert(&claim, (&sender, current_block));
		Self::deposit_event(Event::ClaimCreated {
			who: sender, claim
		});
		Ok(())	
	}

	#[pallet::weight(0)]
	pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {

		let sender = ensure_signed(origin)?;

		let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;
		ensure!(sender == owner, Error::<T>::NotClaimOwner);

		Claims::<T>::remove(&claim);

		Self::deposit_event(Event::ClaimRevoked {
			who: sender, claim
		});

		Ok(())


	}
  }
}