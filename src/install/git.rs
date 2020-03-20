use {
    serde::{Serialize, Deserialize},
    std::fmt::{Display, Formatter}
};

#[derive(Serialize, Deserialize)]
pub struct GitInstall {
    pub repos: Repos
}

#[derive(Serialize, Deserialize)]
pub struct Repos {
    pub user: String,
    pub name: String
}

impl Display for Repos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.user, self.name)
    }
}