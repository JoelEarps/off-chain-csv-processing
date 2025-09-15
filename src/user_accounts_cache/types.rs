use std::collections::HashMap;

use crate::{transaction_handler::types::ClientID, user_accounts_cache::user_accounts::UserAccountDetails};

pub(crate) type AccountStore = HashMap<ClientID, UserAccountDetails>;