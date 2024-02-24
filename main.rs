use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut preguntas_y_respuestas: HashMap<&str, Vec<&str>> = HashMap::new();
    preguntas_y_respuestas.insert("¿Cómo estás?", vec!["Bien, gracias. ¿Y tú?", "Más o menos. ¿Tú?"]);
    preguntas_y_respuestas.insert("¿Qué has hecho hoy?", vec!["He trabajado todo el día. ¿Y tú?", "He estado en casa todo el día. ¿Tú?"]);
    preguntas_y_respuestas.insert("¿Cuál es tu película favorita?", vec!["Me gusta mucho El Padrino. ¿Y tú?", "Me encanta La La Land. ¿Tú?"]);
    preguntas_y_respuestas.insert("¿Qué planes tienes para el fin de semana?", vec!["Tengo una reunión con amigos el sábado. ¿Y tú?", "Todavía no lo he decidido. ¿Tú?"]);
    preguntas_y_respuestas.insert("¿Qué opinas del cambio climático?", vec!["Creo que es una amenaza real para el planeta. ¿Y tú?", "Creo que es un problema importante, pero no estoy seguro de cuál es la mejor manera de abordarlo. ¿Tú?"]);

    println!("Bienvenido a la conversación. Escribe 'bye' para salir.");

    let mut rng = rand::thread_rng();

    loop {
        let mut entrada = String::new();
        println!("Tú: ");
        io::stdin().read_line(&mut entrada).expect("Fallo al leer línea");

        if entrada.trim().eq_ignore_ascii_case("bye") || entrada.trim().eq_ignore_ascii_case("adios") {
            println!("Bot: ¡Hasta luego!");
            break;
        }

        match preguntas_y_respuestas.get(entrada.trim()) {
            Some(respuestas) => {
                let respuesta = respuestas.choose(&mut rng).unwrap();
                println!("Bot: {}", respuesta);
            },
            None => println!("Bot: Lo siento, no entiendo lo que quieres decir."),
        }
    }
}
