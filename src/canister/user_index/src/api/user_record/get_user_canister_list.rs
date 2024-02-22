use candid::Principal;
use ic_cdk_macros::query;
use shared_utils::common::utils::permissions::is_reclaim_canister_id;

use crate::CANISTER_DATA;

#[query(guard = "is_reclaim_canister_id")]
fn get_user_canister_list() -> Vec<Principal> {
    CANISTER_DATA.with(|canister_data_ref_cell| {
        canister_data_ref_cell
            .borrow()
            .user_principal_id_to_canister_id_map
            .values()
            .cloned()
            .collect()
    })
}

#[cfg(test)]
mod test {
    use test_utils::setup::test_constants::{
        get_mock_user_alice_canister_id, get_mock_user_alice_principal_id,
        get_mock_user_bob_canister_id, get_mock_user_bob_principal_id,
    };

    use super::*;

    #[test]
    fn test_get_user_canister_list() {
        CANISTER_DATA.with(|canister_data_ref_cell| {
            let mut canister_data = canister_data_ref_cell.borrow_mut();

            canister_data.user_principal_id_to_canister_id_map.insert(
                get_mock_user_alice_principal_id(),
                get_mock_user_alice_canister_id(),
            );

            canister_data.user_principal_id_to_canister_id_map.insert(
                get_mock_user_bob_principal_id(),
                get_mock_user_bob_canister_id(),
            );
        });

        let user_canister_list = get_user_canister_list();
        assert_eq!(user_canister_list.len(), 2);
        assert!(user_canister_list.contains(&get_mock_user_alice_canister_id()));
        assert!(user_canister_list.contains(&get_mock_user_bob_canister_id()));
    }
}