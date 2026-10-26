use crate::{common::DB, error::Error, models::Validation, traits::DBActions};

use rand::Rng;

pub struct ValidationRepository {
    db: DB,
}

impl ValidationRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    // Generate a random six-digit numeric code.
    fn generate_code() -> String {
        rand::thread_rng().gen_range(100_000..=999_999).to_string()
    }
}

impl DBActions for ValidationRepository {
    type Create = Validation;
    type Update = Validation;
    type Output = Validation;

    async fn create(&self, mut data: Self::Create) -> Result<Self::Output, Error> {
        // Assign a new code before saving.
        data.code = Self::generate_code();

        let validation: Option<Validation> = self.db.create("validation").content(data).await?;

        validation
            .ok_or_else(|| Error::InternalServerError("Failed to create validation".to_string()))
    }

    async fn update(&self, id: &str, mut data: Self::Update) -> Result<Option<Self::Output>, Error> {
        // Assign a new code before saving.

        data.code = Self::generate_code();

        // Replace the existing validation record.

        let validation: Option<Validation> =
            self.db.update(("validation", id)).content(data).await?;

        Ok(validation)
    }

    async fn delete(&self, id: &str) -> Result<bool, Error> {
        // Delete the validation record.
        let validation: Option<Validation> = self.db.delete(("validation", id)).await?;

        Ok(validation.is_some())
    }

    async fn read_one(&self, id: &str) -> Result<Option<Self::Output>, Error> {
        // Find a validation record by its record ID.
        let validation: Option<Validation> = self.db.select(("validation", id)).await?;

        Ok(validation)
    }

    async fn read_all(&self) -> Result<Vec<Self::Output>, Error> {
        // Return all validation records.
        let validations: Vec<Validation> = self.db.select("validation").await?;

        Ok(validations)
    }
}
