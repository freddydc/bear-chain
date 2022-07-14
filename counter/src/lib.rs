use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: i8,
}

#[near_bindgen]
impl Counter {
    pub fn get_result(&self) -> i8 {
        self.value
    }

    pub fn add(&mut self) {
        self.value += 1;
        let message = format!("COUNTER [ {} ]", self.value);
        change_notifier(&message);
    }

    pub fn subtract(&mut self) {
        self.value -= 1;
        let message = format!("COUNTER [ {} ]", self.value);
        change_notifier(&message);
    }

    pub fn reset(&mut self) {
        self.value = 0;
        let message = format!("COUNTER [ {} ]", self.value);
        change_notifier(&message);
        change_notifier("Counter reset to zero");
    }
}

fn change_notifier(message: &str) {
    env::log_str(message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .is_view(is_view)
            .signer_account_id("bob_near".parse().unwrap())
            .build()
    }

    #[test]
    fn add_value() {
        let context = get_context(false);
        testing_env!(context);

        let mut counter = Counter { value: 0 };

        counter.add();
        counter.add();

        assert_eq!(2, counter.get_result());
        assert_eq!(get_logs(), vec!["COUNTER [ 1 ]", "COUNTER [ 2 ]"]);
    }

    #[test]
    fn subtract_value() {
        let context = get_context(false);
        testing_env!(context);

        let mut counter = Counter { value: 0 };

        counter.subtract();

        assert_eq!(-1, counter.get_result());
        assert_eq!(get_logs(), vec!["COUNTER [ -1 ]"]);
    }

    #[test]
    fn reset_value() {
        let context = get_context(false);
        testing_env!(context);

        let mut counter = Counter { value: 10 };

        assert_eq!(10, counter.get_result());

        counter.reset();

        assert_eq!(0, counter.get_result());
        assert_eq!(get_logs(), vec!["COUNTER [ 0 ]", "Counter reset to zero"]);
    }
}
