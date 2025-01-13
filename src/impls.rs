use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId, default_id: [u8; 32]) -> DispatchResult {
		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		
		ensure!(!Kitties::<T>::contains_key(default_id), 
		Error::<T>::DuplicateKitty);


		Kitties::<T>::insert(default_id, ());
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
