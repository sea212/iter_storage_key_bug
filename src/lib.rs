#![cfg(test)]

mod mock;
#[cfg(test)]
mod tests {
    use super::mock::{new_test_ext};
    use frame_support::{
        migration::storage_key_iter,
        storage::StorageDoubleMap as TStorageDoubleMap,
        pallet_prelude::{Identity, Blake2_128Concat, Twox64Concat, StorageDoubleMap},
        ReversibleStorageHasher
    };
    
    const PALLET: &[u8] = b"Pallet";
    const TESTDOUBLEMAP: &[u8] = b"TestDoubleMap";
    
    struct StoragePrefix {}
    
    impl frame_support::traits::StorageInstance for StoragePrefix {
        const STORAGE_PREFIX: &'static str = "TestDoubleMap";
        
        fn pallet_prefix() -> &'static str {
            "Pallet"
        }
    }
    
    fn common_storage_check<S: TStorageDoubleMap<i32, i32, u32>, H: ReversibleStorageHasher>() {
        let key = (-12, -34);
        let value = 56;
        
        S::insert(key.0, key.1, value);
        
        for (k, v) in storage_key_iter::<(i32, i32), u32, H>(&PALLET, &TESTDOUBLEMAP) {
            assert_eq!((key, value), (k, v));
        }
    }
    
    #[test]
    fn test_on_runtime_upgrade_generic_values_with_identity_hasher() {
        new_test_ext().execute_with(|| {
            type TestDoubleMap = StorageDoubleMap<
                StoragePrefix,
                Identity,
                i32,
                Identity,
                i32,
                u32,
            >;
            common_storage_check::<TestDoubleMap, Identity>();
        });
    }
    
    #[test]
    fn test_on_runtime_upgrade_generic_values_with_twox64_concat_hasher() {
        new_test_ext().execute_with(|| {
            type TestDoubleMap = StorageDoubleMap<
                StoragePrefix,
                Twox64Concat,
                i32,
                Twox64Concat,
                i32,
                u32,
            >;
            common_storage_check::<TestDoubleMap, Twox64Concat>();
        });
    }

    #[test]
    fn test_on_runtime_upgrade_generic_values_with_blake2_128concat_hasher() {
        new_test_ext().execute_with(|| {
            type TestDoubleMap = StorageDoubleMap<
                StoragePrefix,
                Blake2_128Concat,
                i32,
                Blake2_128Concat,
                i32,
                u32,
            >;
            common_storage_check::<TestDoubleMap, Blake2_128Concat>();
        });
    }
}
