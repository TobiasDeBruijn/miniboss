pub mod driver;
pub mod user;
pub mod oauth2_client;
mod hash;

use rand::Rng;

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// Macro to show the type of an enum as a String.
/// The enum itself should only implement [sqlx::Encode] and [sqlx::Decode]
// Issue: https://github.com/launchbadge/sqlx/issues/1241
// Comment: https://github.com/launchbadge/sqlx/issues/1241#issuecomment-1649040626
#[macro_export]
macro_rules! impl_enum_type {
    ($ty:ty) => {
        impl sqlx::Type<sqlx::MySql> for $ty {
            fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
                <str as sqlx::Type<sqlx::MySql>>::type_info()
            }

            fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
                <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
            }
        }
    };
}


