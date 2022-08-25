use ic_kit::{candid::CandidType, prelude::*};
use std::collections::HashMap;
use xotp::util::{
    parse_otpauth_uri,
    ParseResult::{self, *},
};

#[derive(Default, CandidType, Deserialize, Serialize)]
pub struct Accounts(Vec<Principal>, HashMap<String, String>);
impl Accounts {
    pub fn custodians(&self) -> Vec<Principal> {
        self.0.clone()
    }

    pub fn add_custodian(&mut self, caller: Principal) {
        self.0.push(caller)
    }

    pub fn remove_custodian(&mut self, caller: Principal) {
        self.0.retain(|x| x != &caller);
    }

    pub fn insert(&mut self, name: String, account: String) {
        self.1.insert(name, account);
    }

    pub fn remove(&mut self, name: String) {
        self.1.remove(&name);
    }

    pub fn get(&self, account: &str) -> Option<ParseResult> {
        match self.1.get(account) {
            Some(x) => Some(parse_otpauth_uri(x).unwrap()),
            None => None,
        }
    }

    pub fn get_otp(&self, account: &str) -> Option<String> {
        match self.get(account) {
            Some(acc) => match acc {
                TOTP(totp) => Some(totp.get_otp(ic::time() / 1_000_000_000).to_string()),
                HOTP(hotp, _) => Some(hotp.get_otp(ic::time() / 1_000_000_000).to_string()),
            },
            None => None,
        }
    }
}

pub fn custodian() -> Result<(), String> {
    match ic::with(Accounts::custodians).contains(&ic::caller()) {
        true => Ok(()),
        false => Err(format!("unauthorized caller")),
    }
}

#[init]
pub fn init(accounts: &mut Accounts) {
    let caller = ic::caller();
    accounts.add_custodian(caller);
}

#[query]
pub fn custodians(accounts: &mut Accounts) -> Vec<Principal> {
    accounts.custodians()
}

#[update(guard = "custodian")]
pub fn add_custodian(accounts: &mut Accounts, user: Principal) {
    accounts.add_custodian(user);
}

#[update(guard = "custodian")]
pub fn remove_custodian(accounts: &mut Accounts, user: Principal) {
    accounts.remove_custodian(user);
}

#[update(guard = "custodian")]
pub fn get_otp(accounts: &mut Accounts, name: String) -> Result<String, String> {
    match accounts.get_otp(&name) {
        Some(x) => Ok(x),
        None => Err("Account not found".to_string()),
    }
}

#[update(guard = "custodian")]
pub fn register_otp(accounts: &mut Accounts, name: String, uri: String) -> Result<(), String> {
    parse_otpauth_uri(uri.as_str()).map_err(|e| format!("{e:?}"))?;
    accounts.insert(name, uri);

    Ok(())
}

#[update(guard = "custodian")]
pub fn remove_otp(accounts: &mut Accounts, name: String) {
    accounts.remove(name);
}

#[pre_upgrade]
pub fn pre_upgrade(accounts: &mut Accounts) {
    ic_kit::stable::stable_store((accounts,));
}

#[post_upgrade]
pub fn post_upgrade() {
    let (accounts,): (Accounts,) = ic_kit::stable::stable_restore().unwrap();
    ic::swap(accounts);
}

#[derive(KitCanister)]
#[candid_path("otp.did")]
pub struct OTPCanister;

#[cfg(test)]
mod tests {
    use super::*;

    #[kit_test]
    async fn test_otp(replica: Replica) {
        let c = replica.add_canister(OTPCanister::anonymous());
        c.init().await;

        let r1 = c
            .new_call("register_otp")
            .with_arg(("test".to_string(), "otpauth://totp/ossian:self@ossian.dev?secret=NICE&issuer=ossian&algorithm=SHA1&digits=6&period=30".to_string()))
            .perform()
            .await
            .decode_one::<Result<(), String>>()
            .unwrap();

        assert_eq!(r1, Ok(()));

        let r2 = c
            .new_call("get_otp")
            .with_arg(("test".to_string(),))
            .perform()
            .await
            .decode_one::<Result<String, String>>()
            .unwrap();

        println!("{:?}", r2);
    }
}
