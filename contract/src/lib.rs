use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

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

impl Default for Contract {
    fn default() -> Self {
        Self {
            insurance_type: String::new(),
            time_period: String::new(),
            user_id: 0,
            user_name: String::new(),
            user_wallet_address: String::new(),
            user_email: String::new(),
            user_phone: String::new(),
            able_to_claim: false,
        }
    }
}

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
            format!(
                "Insurance Type: {}\nTime Period: {}\nUser ID: {}\nUser Name: {}\nUser Wallet Address: {}\nUser Email: {}\nUser Phone: {}\nAble to Claim: {}",
                self.insurance_type,
                self.time_period,
                self.user_id,
                self.user_name,
                self.user_wallet_address,
                self.user_email,
                self.user_phone,
                self.able_to_claim
            )
        } else {
            "User ID does not match".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_insurance() {
        let mut contract = Contract::default();
        contract.buy_insurance(
            "Car Insurance".to_string(),
            "1 year".to_string(),
            1,
            "John Doe".to_string(),
            "0x1234567890".to_string(),
            "johndoe@email.com".to_string(),
            "1234567890".to_string(),
        );
        assert_eq!(
            contract.get_insurance_details(1),
            format!(
                "Insurance Type: {}\nTime Period: {}\nUser ID: {}\nUser Name: {}\nUser Wallet Address: {}\nUser Email: {}\nUser Phone: {}\nAble to Claim: {}",
                "Car Insurance",
                "1 year",
                1,
                "John Doe",
                "0x1234567890",
                "johndoe@email.com",
                "1234567890",
                false
            )
        );
    }
}
