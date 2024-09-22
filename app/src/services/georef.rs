use serde::{Deserialize, Serialize};

const GEOREF: &str = "https://apis.datos.gob.ar/georef/api/direcciones";

#[derive(Deserialize, Serialize)]
struct Altura {
    unidad: Option<String>,
    valor: i16,
}

#[derive(Deserialize, Serialize)]
struct Calle {
    id: String,
    categoria: String,
    nombre: String,
}

#[derive(Deserialize, Serialize)]
struct Departamento {
    id: String,
    nombre: String,
}

#[derive(Deserialize, Serialize)]
struct Localidad {
    id: String,
    nombre: String,
}

#[derive(Deserialize, Serialize)]
struct Provincia {
    id: String,
    nombre: String,
}

#[derive(Deserialize, Serialize)]
pub struct Coordenadas {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Direccion {
    altura: Altura,
    departamento: Option<Departamento>,
    calle: Calle,
    localidad_censal: Localidad,
    nomenclatura: String,
    piso: Option<String>,
    provincia: Provincia,
    pub ubicacion: Coordenadas,
}

#[derive(Deserialize, Serialize)]
pub struct GeoRefIn {
    pub cantidad: i16,
    pub direcciones: Direccion,
}

pub fn request_georef(
    calle: String,
    altura: i16,
    provincia: Option<String>,
) -> Result<ureq::Response, ureq::Error> {
    let direccion = calle + &altura.to_string();

    let query_params: [(&str, &str); 3] = [
        ("direccion", &direccion),
        (
            "provincia",
            match provincia {
                Some(p) => &p.clone(),
                None => "",
            },
        ),
        ("max", "1"),
    ];

    ureq::get(GEOREF).query_pairs(query_params).call()
}
