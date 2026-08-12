use rocket::serde::json::Json;
use crate::models::recurso::Recurso;
use crate::servicos::recurso_servico;
use crate::dtos::recurso_dto::RecursoDto;


#[get("/recursos")]
pub fn index()->Json<Vec<Recurso>>{
    let recursos = recurso_servico::lista_de_recursos();
    Json(recursos)
}

#[post("/recursos", data = "<recurso_dto>")]
pub fn criar(recurso_dto:Json<RecursoDto>)->Json<Vec<Recurso>>{
    let recurso = recurso_dto.into_inner();

    if recurso_servico::cadastrar_recurso(recurso){
       Json(recurso)
    }else{
        
    }
}
