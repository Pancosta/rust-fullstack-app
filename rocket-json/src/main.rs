
#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

struct Recurso{
    id: u32,
    titulo: String,
    descricao: String,
}

#[derive(Serialize)]
#[serde{crate = "rocket::serde" }]

struct Home{  
    mensagem: String,
    endpoints: Vec<String>
}

#[get("/")]
fn home()->Json<Home>{
    Json(Home{
        mensagem:"bem vindo a API".to_string(),
        endpoints: vec![
            "/recursos".to_string()
        ],
    })
}
#[get("/recursos")]
fn recursos_index()->Json<Vec<Recurso>>{
    let recursos = vec![
        Recurso{id:1,titulo: "Recurso 1".to_string(), descricao:"descricao do Recurso 1".to_string()},
        Recurso{id:2,titulo: "Recurso 2".to_string(), descricao:"descricao do Recurso 2".to_string()}
    ];

    Json(recursos)
}

#[launch]
fn rocket()->_ {
    rocket::build().mount("/", routes![
        home,
        recursos_index
    ])
}
