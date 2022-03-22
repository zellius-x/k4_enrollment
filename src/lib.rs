use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};
use std::string::String;
use std::option::Option;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct K4Enrollment {
    records: LookupMap<String, String>,
}

impl Default for K4Enrollment {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl K4Enrollment {
    pub fn enroll(&mut self, account_id: String, enroll_message: String) {
        env::log(format!("Tài khoản: {} đã đăng ký thành công: {}", account_id, enroll_message).as_bytes());
        self.records.insert(&account_id, &enroll_message);
    }

    pub fn get_enroll_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn delete_enroll_record(&mut self, account_id: String) {
        env::log(format!("Khoá học: {} của tài khoản: {} đã được huỷ thành công!", self.records.get(&account_id).unwrap(), account_id).as_bytes());
        
        self.records.remove(&account_id);
    }
}