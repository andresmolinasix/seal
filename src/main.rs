use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct PreguntasOpcion {
    pregunta: Vec<String>,
    opciona: Vec<String>,
    opcionb: Vec<String>,
    opcionc: Vec<String>,
    opciond: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PremioCorrect {
    premio: Vec<f64>,
    correcto: Vec<String>,  // Cambiado a Vec<String> para coincidir con el JSON
}

fn main() -> Result<(), Box<dyn Error>> {
    // Primera pregunta hardcodeada
    println!("\n ¿Cuál es la capital de Colombia?");
    println!("A. Caracas           B. Cali");
    println!("C. Buenos Aires      D. Bogotá");
    
    println!("\n Ingrese su respuesta (letra):");
    let mut parametro1 = String::new();
    io::stdin().read_line(&mut parametro1).expect("Error en lectura");
    
    let respuesta = pregunta_colombia(parametro1.trim());
    println!("Respuesta correcta: {}", respuesta);

    // Segunda pregunta usando la función crear_pregunta
    crear_pregunta("¿Cuál es la capital de Cauca?", "Popayán", "Cali", "Neiva", "Bogotá");

    // Leer archivo de preguntas
    match fs::read_to_string("data/preguntas.json") {
        Ok(data) => {
            // Deserializar preguntas
            match serde_json::from_str::<PreguntasOpcion>(&data) {
                Ok(preguntas) => {
                    // Leer archivo de respuestas
                    let respuestas: PremioCorrect = match fs::read_to_string("data/respuestas.json") {
                        Ok(data_resp) => serde_json::from_str(&data_resp)?,
                        Err(e) => {
                            println!("Error al leer archivo de respuestas: {}", e);
                            return Ok(());
                        }
                    };

                    // Mostrar preguntas y procesar respuestas
                    for (i, pregunta) in preguntas.pregunta.iter().enumerate() {
                        println!("\nPregunta {}: {}", i + 1, pregunta);
                        println!("Opción A: {}", preguntas.opciona[i]);
                        println!("Opción B: {}", preguntas.opcionb[i]);
                        println!("Opción C: {}", preguntas.opcionc[i]);
                        println!("Opción D: {}", preguntas.opciond[i]);
                    
                        println!("\nIngrese su respuesta (letra):");
                        let mut respuesta_usuario = String::new();
                        io::stdin().read_line(&mut respuesta_usuario)?;
                        
                        let es_correcta = respuesta_fn(
                            respuesta_usuario.trim(), 
                            &respuestas.correcto[i]
                        );

                        if es_correcta {
                            println!("¡Correcto! Has ganado ${}", respuestas.premio[i]);
                        } else {
                            println!("Incorrecto. La respuesta correcta era: {}", respuestas.correcto[i]);
                            break;
                        }
                    }
                }
                Err(e) => println!("Error al deserializar preguntas: {}", e),
            }
        }
        Err(e) => println!("Error al leer el archivo de preguntas: {}", e),
    }

    Ok(())
}

fn pregunta_colombia(opcion: &str) -> bool {
    opcion.chars().next() == Some('D') || opcion.chars().next() == Some('d')
}

fn crear_pregunta(pregunta: &str, a: &str, b: &str, c: &str, d: &str) {
    println!("\n {}", pregunta);
    println!("A. {}            B. {}", a, b);
    println!("C. {}            D. {}", c, d);
}

fn respuesta_fn(input: &str, respuesta: &str) -> bool {
    input.trim().to_uppercase() == respuesta.trim().to_uppercase()
}