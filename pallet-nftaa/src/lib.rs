#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_system::Config as SystemConfig;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// The log target of this pallet.
pub const LOG_TARGET: &'static str = "runtime::nftaa";

/// A type alias for the account ID type used in the dispatchable functions of this pallet.
type AccountIdLookupOf<T> = <<T as SystemConfig>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_nfts::{
        AttributeNamespace, CollectionSettings, DestroyWitness, MintSettings, MintWitness,
    };
    use types::{BalanceOf, CollectionConfigFor, DepositBalanceOf, ItemPrice};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config + pallet_nfts::Config<I> {
        /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
        type RuntimeEvent: From<Event<Self, I>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeCall: From<Call<Self, I>> + Encode;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    /// Pallets use events to inform users when important changes are made.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {}

    /// Errors inform users that something went wrong.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::error]
    pub enum Error<T, I = ()> {}

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables>

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// Issue a new collection of non-fungible items from a public origin.
        ///
        /// This new collection has no items initially and its owner is the origin.
        ///
        /// The origin must be Signed and the sender must have sufficient funds free.
        ///
        /// `CollectionDeposit` funds of sender are reserved.
        ///
        /// Parameters:
        /// - `admin`: The admin of this collection. The admin is the initial address of each
        /// member of the collection's admin team.
        ///
        /// Emits `Created` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            admin: AccountIdLookupOf<T>,
            config: CollectionConfigFor<T, I>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::create(origin, admin, config)
        }

        /// Destroy a collection of fungible items.
        ///
        /// The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the
        /// owner of the `collection`.
        ///
        /// NOTE: The collection must have 0 items to be destroyed.
        ///
        /// - `collection`: The identifier of the collection to be destroyed.
        /// - `witness`: Information on the items minted in the collection. This must be
        /// correct.
        ///
        /// Emits `Destroyed` event when successful.
        ///
        /// Weight: `O(m + c + a)` where:
        /// - `m = witness.item_metadatas`
        /// - `c = witness.item_configs`
        /// - `a = witness.attributes`
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn destroy(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            witness: DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            pallet_nfts::Pallet::<T, I>::destroy(origin, collection, witness)
        }

        /// Mint an item of a particular collection.
        ///
        /// The origin must be Signed and the sender must comply with the `mint_settings` rules.
        ///
        /// - `collection`: The collection of the item to be minted.
        /// - `item`: An identifier of the new item.
        /// - `mint_to`: Account into which the item will be minted.
        /// - `witness_data`: When the mint type is `HolderOf(collection_id)`, then the owned
        ///   item_id from that collection needs to be provided within the witness data object. If
        ///   the mint price is set, then it should be additionally confirmed in the `witness_data`.
        ///
        /// Note: the deposit will be taken from the `origin` and not the `owner` of the `item`.
        ///
        /// Emits `Issued` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn mint(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            mint_to: AccountIdLookupOf<T>,
            witness_data: Option<MintWitness<T::ItemId, DepositBalanceOf<T, I>>>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::mint(origin, collection, item, mint_to, witness_data)
        }

        /// Destroy a single item.
        ///
        /// The origin must conform to `ForceOrigin` or must be Signed and the signing account must
        /// be the owner of the `item`.
        ///
        /// - `collection`: The collection of the item to be burned.
        /// - `item`: The item to be burned.
        ///
        /// Emits `Burned`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn burn(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::burn(origin, collection, item)
        }

        /// Move an item from the sender account to another.
        ///
        /// Origin must be Signed and the signing account must be either:
        /// - the Owner of the `item`;
        /// - the approved delegate for the `item` (in this case, the approval is reset).
        ///
        /// Arguments:
        /// - `collection`: The collection of the item to be transferred.
        /// - `item`: The item to be transferred.
        /// - `dest`: The account to receive ownership of the item.
        ///
        /// Emits `Transferred`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            dest: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::transfer(origin, collection, item, dest)
        }

        /// Set an attribute for a collection or item.
        ///
        /// Origin must be Signed and must conform to the namespace ruleset:
        /// - `CollectionOwner` namespace could be modified by the `collection` Admin only;
        /// - `ItemOwner` namespace could be modified by the `maybe_item` owner only. `maybe_item`
        ///   should be set in that case;
        /// - `Account(AccountId)` namespace could be modified only when the `origin` was given a
        ///   permission to do so;
        ///
        /// The funds of `origin` are reserved according to the formula:
        /// `AttributeDepositBase + DepositPerByte * (key.len + value.len)` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to set.
        /// - `maybe_item`: The identifier of the item whose metadata to set.
        /// - `namespace`: Attribute's namespace.
        /// - `key`: The key of the attribute.
        /// - `value`: The value to which to set the attribute.
        ///
        /// Emits `AttributeSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(19)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_attribute(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            namespace: AttributeNamespace<T::AccountId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_attribute(
                origin, collection, maybe_item, namespace, key, value,
            )
        }

        /// Clear an attribute for a collection or item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
        /// attribute.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to clear.
        /// - `maybe_item`: The identifier of the item whose metadata to clear.
        /// - `namespace`: Attribute's namespace.
        /// - `key`: The key of the attribute.
        ///
        /// Emits `AttributeCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(21)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn clear_attribute(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            maybe_item: Option<T::ItemId>,
            namespace: AttributeNamespace<T::AccountId>,
            key: BoundedVec<u8, T::KeyLimit>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::clear_attribute(
                origin, collection, maybe_item, namespace, key,
            )
        }

        /// Allows to buy an item if it's up for sale.
        ///
        /// Origin must be Signed and must not be the owner of the `item`.
        ///
        /// - `collection`: The collection of the item.
        /// - `item`: The item the sender wants to buy.
        /// - `bid_price`: The price the sender is willing to pay.
        ///
        /// Emits `ItemBought` on success.
        #[pallet::call_index(32)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1, 1))]
        pub fn buy_item(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            bid_price: ItemPrice<T, I>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::buy_item(origin, collection, item, bid_price)
        }

        /// Clear the metadata for a collection.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Admin of
        /// the `collection`.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose metadata to clear.
        ///
        /// Emits `CollectionMetadataCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(27)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn clear_collection_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::clear_collection_metadata(origin, collection)
        }

        /// Clear the metadata for an item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin of the
        /// `collection`.
        ///
        /// Any deposit is freed for the collection's owner.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to clear.
        /// - `item`: The identifier of the item whose metadata to clear.
        ///
        /// Emits `ItemMetadataCleared`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(25)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn clear_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::clear_metadata(origin, collection, item)
        }

        /// Disallows specified settings for the whole collection.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection to be locked.
        /// - `lock_settings`: The settings to be locked.
        ///
        /// Note: it's possible to only lock(set) the setting, but not to unset it.
        ///
        /// Emits `CollectionLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(10)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn lock_collection(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            lock_settings: CollectionSettings,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::lock_collection(origin, collection, lock_settings)
        }

        /// Disallows changing the metadata or attributes of the item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin
        /// of the `collection`.
        ///
        /// - `collection`: The collection if the `item`.
        /// - `item`: An item to be locked.
        /// - `lock_metadata`: Specifies whether the metadata should be locked.
        /// - `lock_attributes`: Specifies whether the attributes in the `CollectionOwner` namespace
        ///   should be locked.
        ///
        /// Note: `lock_attributes` affects the attributes in the `CollectionOwner` namespace only.
        /// When the metadata or attributes are locked, it won't be possible the unlock them.
        ///
        /// Emits `ItemPropertiesLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(18)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn lock_item_properties(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            lock_metadata: bool,
            lock_attributes: bool,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::lock_item_properties(
                origin,
                collection,
                item,
                lock_metadata,
                lock_attributes,
            )
        }

        /// Disallow further unprivileged transfer of an item.
        ///
        /// Origin must be Signed and the sender should be the Freezer of the `collection`.
        ///
        /// - `collection`: The collection of the item to be changed.
        /// - `item`: The item to become non-transferable.
        ///
        /// Emits `ItemTransferLocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(8)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn lock_item_transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::lock_item_transfer(origin, collection, item)
        }

        /// Re-evaluate the deposits on some items.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection of the items to be reevaluated.
        /// - `items`: The items of the collection whose deposits will be reevaluated.
        ///
        /// NOTE: This exists as a best-effort function. Any items which are unknown or
        /// in the case that the owner account does not have reservable funds to pay for a
        /// deposit increase are ignored. Generally the owner isn't going to call this on items
        /// whose existing deposit is less than the refreshed deposit as it would only cost them,
        /// so it's of little consequence.
        ///
        /// It will still return an error in the case that the collection is unknown or the signer
        /// is not permitted to call it.
        ///
        /// Weight: `O(items.len())`
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1, 1))]
        pub fn redeposit(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            items: Vec<T::ItemId>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::redeposit(origin, collection, items)
        }

        /// Set the maximum number of items a collection could have.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of
        /// the `collection`.
        ///
        /// - `collection`: The identifier of the collection to change.
        /// - `max_supply`: The maximum number of items a collection could have.
        ///
        /// Emits `CollectionMaxSupplySet` event when successful.
        #[pallet::call_index(29)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_collection_max_supply(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            max_supply: u32,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_collection_max_supply(origin, collection, max_supply)
        }

        /// Set the metadata for a collection.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Admin of
        /// the `collection`.
        ///
        /// If the origin is `Signed`, then funds of signer are reserved according to the formula:
        /// `MetadataDepositBase + DepositPerByte * data.len` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the item whose metadata to update.
        /// - `data`: The general information of this item. Limited in length by `StringLimit`.
        ///
        /// Emits `CollectionMetadataSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(26)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_collection_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            data: BoundedVec<u8, T::StringLimit>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_collection_metadata(origin, collection, data)
        }

        /// Set the metadata for an item.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Admin of the
        /// `collection`.
        ///
        /// If the origin is Signed, then funds of signer are reserved according to the formula:
        /// `MetadataDepositBase + DepositPerByte * data.len` taking into
        /// account any already reserved funds.
        ///
        /// - `collection`: The identifier of the collection whose item's metadata to set.
        /// - `item`: The identifier of the item whose metadata to set.
        /// - `data`: The general information of this item. Limited in length by `StringLimit`.
        ///
        /// Emits `ItemMetadataSet`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(24)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_metadata(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            data: BoundedVec<u8, T::StringLimit>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_metadata(origin, collection, item, data)
        }

        /// Set (or reset) the price for an item.
        ///
        /// Origin must be Signed and must be the owner of the `item`.
        ///
        /// - `collection`: The collection of the item.
        /// - `item`: The item to set the price for.
        /// - `price`: The price for the item. Pass `None`, to reset the price.
        /// - `buyer`: Restricts the buy operation to a specific account.
        ///
        /// Emits `ItemPriceSet` on success if the price is not `None`.
        /// Emits `ItemPriceRemoved` on success if the price is `None`.
        #[pallet::call_index(31)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_price(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
            price: Option<ItemPrice<T, I>>,
            whitelisted_buyer: Option<AccountIdLookupOf<T>>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_price(
                origin,
                collection,
                item,
                price,
                whitelisted_buyer,
            )
        }

        /// Change the Issuer, Admin and Freezer of a collection.
        ///
        /// Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the
        /// `collection`.
        ///
        /// Note: by setting the role to `None` only the `ForceOrigin` will be able to change it
        /// after to `Some(account)`.
        ///
        /// - `collection`: The collection whose team should be changed.
        /// - `issuer`: The new Issuer of this collection.
        /// - `admin`: The new Admin of this collection.
        /// - `freezer`: The new Freezer of this collection.
        ///
        /// Emits `TeamChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(12)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_team(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            issuer: Option<AccountIdLookupOf<T>>,
            admin: Option<AccountIdLookupOf<T>>,
            freezer: Option<AccountIdLookupOf<T>>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::set_team(origin, collection, issuer, admin, freezer)
        }

        /// Change the Owner of a collection.
        ///
        /// Origin must be Signed and the sender should be the Owner of the `collection`.
        ///
        /// - `collection`: The collection whose owner should be changed.
        /// - `owner`: The new Owner of this collection. They must have called
        ///   `set_accept_ownership` with `collection` in order for this operation to succeed.
        ///
        /// Emits `OwnerChanged`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(11)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn transfer_ownership(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            new_owner: AccountIdLookupOf<T>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::transfer_ownership(origin, collection, new_owner)
        }

        /// Re-allow unprivileged transfer of an item.
        ///
        /// Origin must be Signed and the sender should be the Freezer of the `collection`.
        ///
        /// - `collection`: The collection of the item to be changed.
        /// - `item`: The item to become transferable.
        ///
        /// Emits `ItemTransferUnlocked`.
        ///
        /// Weight: `O(1)`
        #[pallet::call_index(9)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn unlock_item_transfer(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::ItemId,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::unlock_item_transfer(origin, collection, item)
        }

        /// Update mint settings.
        ///
        /// Origin must be either `ForceOrigin` or `Signed` and the sender should be the Issuer
        /// of the `collection`.
        ///
        /// - `collection`: The identifier of the collection to change.
        /// - `mint_settings`: The new mint settings.
        ///
        /// Emits `CollectionMintSettingsUpdated` event when successful.
        #[pallet::call_index(30)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn update_mint_settings(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            mint_settings: MintSettings<BalanceOf<T, I>, BlockNumberFor<T>, T::CollectionId>,
        ) -> DispatchResult {
            pallet_nfts::Pallet::<T, I>::update_mint_settings(origin, collection, mint_settings)
        }
    }
}
