#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{KeyedVec, EncodeLike};
use frame_support::{pallet_prelude::{*, OptionQuery}, traits::fungibles};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;
	use scale_info::prelude::string::String;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn json_data)]
	pub type DataStorage<T> = StorageValue<_, Vec<u8>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_score)]
	pub type GetScore<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, u32, ValueQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ESGStored(Vec<u8>, T::AccountId),
		ScoreGet{company_id: T::AccountId, score: u32},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn esgscore(origin: OriginFor<T>, json_data: Vec<u8>) -> DispatchResult {
		
			let signer = ensure_signed(origin)?;
			let data = <DataStorage<T>>::put(&json_data);
			// frame_support::log::info!("json data **************** {:?}", data);
			let abc = <DataStorage<T>>::get();
			
			// let acc = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" as &EncodeLike<T>;
			// let sc:u32 = 55;

			let xyz =match String::from_utf8(json_data.clone()) {
				Ok(path) => Ok(path),
				Err(e) => Err(e),
			};
			frame_support::log::info!("data ******************** {:?}", xyz);
			// for item in xyz.unwrap().split_ascii_whitespace() {
			// 	let splits = item.split(",").collect::<Vec<&str>>();
			// 	frame_support::log::info!("split data **************** key: {}, value: {}", splits[0], splits[1]);
			// 	// <GetScore<T>>::insert(splits[0], splits[1]);
			// }

			frame_support::log::info!("data ******************** {:?}", xyz.unwrap().split_whitespace().map(|s| s.split_at(s.find(":").unwrap())).map(|(key, val)| (key, &val[1..])).collect::<T>());
			// let aaa:GetScore<T> = xyz.unwrap().split_whitespace().map(|s| s.split_at(s.find(":").unwrap())).map(|(key, val)| (key, &val[1..])).collect();
			// frame_support::log::info!("expected value **********************#### {:#?}", aaa);
			// for i in abc {
			// 	<GetScore<T>>::insert(abc.split[0], abc.split[1])
			// }
			<GetScore<T>>::insert(acc,sc);
			Self::deposit_event(Event::ESGStored(json_data, signer.clone()));
			Ok(())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn score_get(origin: OriginFor<T>, score: u32) -> DispatchResult {
		// 	let signer = ensure_signed(origin)?;
		// 	<GetScore<T>>::insert(signer.clone(), score);
		// 	Self::deposit_event(Event::ScoreGet{ score: score, company_id: signer.clone()});			
		// 	Ok(())
		// }

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn get_account_score(origin: OriginFor<T>) -> DispatchResult {
		// 	// let signer = ensure_signed(origin)?;
		// 	// let abc = <DAa<T>>::get(&signer);
		// 	// Self::deposit_event(Event::ScoreGet{ score: abc, company_id: signer.clone()});	
		// 	let abc = <DataStorage<T>>::get().unwrap();
		// 	let xyz =match String::from_utf8(abc.clone()) {
		// 		Ok(path) => Ok(path),
		// 		Err(e) => Err(e),
		// 	};
		// 	frame_support::log::info!("data ******************** {:?}", xyz);
		// 	frame_support::log::info!("unwrap data ******************** {:?}", xyz.unwrap());		
		// 	Ok(())
		// }
		
	}

	// impl<T:Config> fungibles::EsgScore<T::AccountId> for Pallet<T> {
	// 	fn esg_score(company: T::AccountId) -> u32 {
	// 		GetScore::<T>::get(&company)
	// 	}
	// }

}
