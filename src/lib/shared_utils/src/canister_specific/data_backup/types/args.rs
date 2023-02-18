use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};

use crate::{access_control::UserAccessRole, common::types::known_principal::KnownPrincipalMapV1};

#[derive(Deserialize, CandidType, Default)]
pub struct DataBackupInitArgs {
    pub known_principal_ids: Option<KnownPrincipalMapV1>,
    pub access_control_map: Option<HashMap<Principal, Vec<UserAccessRole>>>,
}