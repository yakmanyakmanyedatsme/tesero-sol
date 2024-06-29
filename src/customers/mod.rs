pub mod individuals;
pub mod municipals;
pub mod accounts {
    use super::individuals::Individual;
    use crate::general::address::Address;
    use crate::general::contactinfo::ContactInfo;
    use crate::general::credentials::Credentials;
    use chrono::Utc;

    impl Individual {
        pub fn new(cred: Credentials, contact: ContactInfo, address: Address) -> Self {
            Self {
                cred,
                contact,
                address: Some(address),
                sign_up_date: Some(Utc::now().naive_utc().into()),
                maintenance: None,
                installations: None,
            }
        }

        pub fn default() -> Self {
            Self {
                cred: Credentials {
                    email: String::from("default@example.com"),
                    password: String::from("defaultpassword"),
                },
                contact: ContactInfo {
                    first: String::from("Default"),
                    last: String::from("User"),
                    middle_name: None,
                    phone_number: String::from("000-000-0000"),
                    email: String::from("default@example.com"),
                },
                address: None,
                sign_up_date: None,
                maintenance: None,
                installations: None,
            }
        }

        pub fn set_credentials(&mut self, email: String, password: String) {
            self.cred = Credentials { email, password };
        }
    }
}
