use crate::{
    common::DB,
    error::Error,
    models::{UpdateUser, User, UserResponse},
    traits::DBActions,
};

pub struct UserRepository {
    db: DB,
}

impl UserRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

// implement DBActions trait for UserRepository and rewrite functions
impl DBActions for UserRepository {
    type Create = User;
    type Update = UpdateUser;
    type Output = UserResponse;

    // create user with user model template
    async fn create(&self, data: Self::Create) -> Result<Self::Output, Error> {
        let user: Option<User> = self.db.create("user").content(data).await?;

        let user =
            user.ok_or_else(|| Error::InternalServerError("Failed to create user".to_string()))?;

        Ok(UserResponse::from(user))
    }

    // update user with id and updateuser template
    async fn update(&self, id: &str, data: Self::Update) -> Result<Option<Self::Output>, Error> {
        let user: Option<User> = self.db.update(("user", id)).content(data).await?;

        Ok(user.map(UserResponse::from))
    }

    // delete user by id
    async fn delete(&self, id: &str) -> Result<bool, Error> {
        let user: Option<User> = self.db.delete(("user", id)).await?;

        Ok(user.is_some())
    }

    // read user by id and return
    async fn read_one(&self, id: &str) -> Result<Option<Self::Output>, Error> {
        let user: Option<User> = self.db.select(("user", id)).await?;

        Ok(user.map(UserResponse::from))
    }

    // read all users and return vector of UserResponse
    async fn read_all(&self) -> Result<Vec<Self::Output>, Error> {
        let users: Vec<User> = self.db.select("user").await?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }
}
