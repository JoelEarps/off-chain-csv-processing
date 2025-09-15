use std::collections::HashMap;

use crate::{transaction_event_handler::transaction_event_handler::ClientID, user_accounts_cache::user_accounts::UserAccountDetails};

pub(crate) type AccountStore = HashMap<ClientID, UserAccountDetails>;