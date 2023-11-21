use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    texto: String,
    tag: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().to_string(),
            tag: row.get(1).unwrap().to_string(),
            texto: row.get(2).unwrap().to_string(),
            vida: row.get(3).unwrap().parse::<i32>().unwrap_or(0),
            opciones: Vec::new(),
        };
    }
}

fn main() {
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();
    let mut last_record: String = "".to_string();
    let mut tag_actual = FIRST_TAG.to_string();

    let content = fs::read_to_string(FILENAME).expect("No se pudo leer el archivo");

    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in reader.records() {
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);
        if dato.tipo_dato == "SITUACION" {
            let record_tag = dato.tag.clone();
            datos_historia.insert(record_tag.clone(), dato);
            last_record = record_tag
        } else if dato.tipo_dato == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record) {
                (*data).opciones.push(dato);
            }
        }
    }

    println!("{:?}", datos_historia);
}
