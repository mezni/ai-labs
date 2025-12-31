use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, validator::ValidationErrors> {
        let email = Self { value };
        email.validate()?;
        Ok(email)
    }
}

#[derive(Debug, Clone, Validate)]
pub struct Username {
    #[validate(length(min = 3, max = 100))]
    pub value: String,
}

impl Username {
    pub fn new(value: String) -> Result<Self, validator::ValidationErrors> {
        let username = Self { value };
        username.validate()?;
        Ok(username)
    }
}
