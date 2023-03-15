use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // Obtener de la línea de comandos
    // el nombre del archivo de entrada
    // ya que se encuentra en primer argumento
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut contador_palabras = HashMap::new();

    // abro el archivo
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // por cada linea, la agrego al mapa
    for line in reader.lines() {
        for word in line?.split_whitespace() {
            *contador_palabras.entry(word.to_owned()).or_insert(0) += 1;
        }
    }

    // creo un vector a partir del map
    let mut word_count_vec: Vec<_> = contador_palabras.into_iter().collect();

    // ordeno el vector de mayor a menor frecuencia
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // imprimo
    for (word, count) in word_count_vec {
        println!("{} -> {}", word, count);
    }

    Ok(())
}
