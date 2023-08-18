// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    insurance_type: String,
    time_period: String,
    user_id: u128,
    user_name: String,
    user_wallet_address: String,
    user_email: String,
    user_phone: String,
    able_to_claim: bool,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        // Self{message: DEFAULT_MESSAGE.to_string()}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn buy_insurance(
        &mut self,
        insurance_type: String,
        time_period: String,
        user_id: u128,
        user_name: String,
        user_wallet_address: String,
        user_email: String,
        user_phone: String,
    ) {
        self.insurance_type = insurance_type;
        self.time_period = time_period;
        self.user_id = user_id;
        self.user_name = user_name;
        self.user_wallet_address = user_wallet_address;
        self.user_email = user_email;
        self.user_phone = user_phone;
        self.able_to_claim = false;
    }

    pub fn claim_insurance(&mut self, user_id: u128, attestation: bool) {
        if self.user_id == user_id {
            self.able_to_claim = attestation;
        }
    }

    pub fn get_insurance_details(&self, user_id: u128) -> String {
        if self.user_id == user_id {
            return format!(
                "Insurance Type: {}\nTime Period: {}\nUser ID: {}\nUser Name: {}\nUser Wallet Address: {}\nUser Email: {}\nUser Phone: {}\nAble to Claim: {}",
                self.insurance_type, self.time_period, self.user_id, self.user_name, self.user_wallet_address, self.user_email, self.user_phone, self.able_to_claim
            );
        } else {
            return "User ID does not match".to_string();
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_insurance() {
        let contract = Contract::default();
        contract.buy_insurance(
            "Car Insurance".to_string(),
            "1 year".to_string(),
            1,
            "John Doe".to_string(),
            "0x1234567890".to_string(),
            "johndoe@email.com".to_string(),
            "1234567890".to_string(),
        );
        assert_eq!(contract.get_insurance_details(1), format!("Insurance Type: {}\nTime Period: {}\nUser ID: {}\nUser Name: {}\nUser Wallet Address: {}\nUser Email: {}\nUser Phone: {}\nAble to Claim: {}", "Car Insurance".to_string(), "1 year".to_string(), 1, "John Doe".to_string(), "0x1234567890".to_string(), "johndoe@email.com".to_string(),"1234567890".to_string()));
    }
}
