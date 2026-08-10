use rocket::serde::json::Json;
use crate::model_views::home::Home;


#[get("/")]
pub fn index()->Json<Home>{
    Json(Home{
        mensagem:"bem vindo a API".to_string(),
        endpoints: vec![
            "/recursos".to_string()
        ],
    })
}