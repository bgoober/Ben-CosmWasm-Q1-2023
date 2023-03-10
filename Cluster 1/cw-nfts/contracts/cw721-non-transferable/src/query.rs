use cosmwasm_std::{Deps, StdResult};

use crate::{msg::AdminResponse, state::CONFIG};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(AdminResponse {
        admin: config.admin.map(|admin| admin.to_string()),
    })
}
