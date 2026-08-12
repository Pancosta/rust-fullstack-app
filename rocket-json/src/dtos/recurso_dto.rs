use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

pub struct RecursoDto{
    pub titulo: String,
    pub descricao: String,
}