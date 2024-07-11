// TODO: Re-implement `Ticket`'s accessor methods. This time return a `&str` rather than a `&String`.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Self::validate_title(&title);
        Self::validate_description(&description);
        Self::validate_status(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn set_title(&mut self, title: String){
        Self::validate_title(&title);
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        Self::validate_description(&description);
        self.description = description;
    }

    pub fn set_status(&mut self, status: String) {
        Self::validate_status(&status);
        self.status = status;
    }

    fn validate_status(status: &String){
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }

    fn validate_description(description: &String){
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
    }

    fn validate_title(title: &String){
        if title.is_empty() {
            panic!("Title cannot be empty");
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // Some dark magic to verify that you used the expected return types
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
