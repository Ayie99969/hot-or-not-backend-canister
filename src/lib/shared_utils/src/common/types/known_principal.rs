use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum KnownPrincipalType {
    UserIdGlobalSuperAdmin,
    CanisterIdConfiguration,
    CanisterIdDataBackup,
    CanisterIdPostCache,
    CanisterIdProjectMemberIndex,
    CanisterIdRootCanister,
    CanisterIdSNSController,
    CanisterIdTopicCacheIndex,
    CanisterIdUserIndex,
}

pub type KnownPrincipalMap = HashMap<KnownPrincipalType, Principal>;
