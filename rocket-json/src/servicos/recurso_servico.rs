use crate::models::recurso::Recurso;
use crate::dtos::recurso_dto::RecursoDto;


pub fn lista_de_recursos() -> Vec<Recurso>{
    // buscando dados do banco
    return vec![
        Recurso{id:1,titulo: "Recurso 1".to_string(), descricao:"descricao do Recurso 1".to_string()},
        Recurso{id:2,titulo: "Recurso 2".to_string(), descricao:"descricao do Recurso 2".to_string()}
    ];

}

pub fn cadastrar_recurso(recurso: RecursoDto) -> Result<Recurso, String>{
    // usar repositório para gravar no DB
    println!("Titulo: {}", recurso.titulo);
    println!("Descrição: {}", recurso.descricao);

    if true {
        Ok((Recurso{}))
    } else {
        Err("Erro ao Cadastrar".to_string())
    }
}