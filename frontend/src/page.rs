pub mod register;
pub use register::*;
pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
}
