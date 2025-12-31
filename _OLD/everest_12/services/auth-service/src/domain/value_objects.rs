use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct Email {
    #[validate(email)]
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, validator::ValidationErrors> {
        let email = Self { value };
        email.validate()?;
        Ok(email)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl From<Email> for String {
    fn from(e: Email) -> Self {
        e.value
    }
}

#[derive(Debug, Clone, Validate)]
pub struct Username {
    #[validate(length(min = 3, max = 100))]
    value: String,
}

impl Username {
    pub fn new(value: String) -> Result<Self, validator::ValidationErrors> {
        let username = Self { value };
        username.validate()?;
        Ok(username)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl From<Username> for String {
    fn from(u: Username) -> Self {
        u.value
    }
}
