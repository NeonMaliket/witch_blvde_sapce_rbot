use lazy_static::lazy_static;
use std::env;
use std::fmt::{Debug, Display};
use supabase_rs::SupabaseClient;

lazy_static! {
    static ref SUPABASE_CLIENT: SupabaseClient = {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
        SupabaseClient::new(url, key).expect("Supabase client initialisation error")
    };
}

pub fn get_supabase_config() -> &'static SupabaseClient {
    &SUPABASE_CLIENT
}
