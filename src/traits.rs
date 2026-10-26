use crate::error::Error;

/// Generic CRUD contract every repository implements.
///
/// Uses associated types instead of generic parameters because each
/// repository has exactly one create/update model and one output model;
/// this keeps `impl` blocks and call sites readable.
///
/// Note: async trait methods make this trait not dyn-compatible
/// (object-safe). Use it via generics, e.g. `impl<T: DBActions> ...`.
pub trait DBActions {
    /// Model accepted when creating a new record.
    type Create;

    /// Model accepted when updating an existing record.
    type Update;

    /// Public model returned to callers (never leaks internals
    /// such as password hashes).
    type Output;

    /// Create a record from `Create` and return the sanitized `Output`.
    ///
    /// # Errors
    /// Returns [`Error`] if the insert fails or the driver returns no record.
    async fn create(&self, data: Self::Create) -> Result<Self::Output, Error>;

    /// Replace the record with the given ID using `Update`.
    ///
    /// Returns `None` when no record exists for `id`.
    async fn update(&self, id: &str, data: Self::Update) -> Result<Option<Self::Output>, Error>;

    /// Delete the record with the given ID.
    ///
    /// Returns `true` only if a record was actually removed.
    async fn delete(&self, id: &str) -> Result<bool, Error>;

    /// Fetch a single record by ID.
    ///
    /// Returns `None` when no record exists for `id`.
    async fn read_one(&self, id: &str) -> Result<Option<Self::Output>, Error>;

    /// Fetch all records.
    async fn read_all(&self) -> Result<Vec<Self::Output>, Error>;
}
