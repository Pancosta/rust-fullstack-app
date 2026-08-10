use rocket::serde::json::Json;
use crate::models::recurso::Recurso;


#[get("/recursos")]
pub fn index()->Json<Vec<Recurso>>{
    let recursos = vec![
        Recurso{id:1,titulo: "Recurso 1".to_string(), descricao:"descricao do Recurso 1".to_string()},
        Recurso{id:2,titulo: "Recurso 2".to_string(), descricao:"descricao do Recurso 2".to_string()}
    ];

    Json(recursos)
}
