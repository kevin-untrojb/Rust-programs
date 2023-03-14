use std::io::{self, Write};


fn main() {
    println!("¡Bienvenido al ahorcado de FIUBA!");
    let mut palabra_secreta = String::new();
    let mut letras_ingresadas_adivinadas= String::new();

    print!("Ingresa una palabra para jugar al ahorcado: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut palabra_secreta).expect("Error leyendo la linea.");

    let mut letras_adivinadas = vec!['_'; palabra_secreta.len()-1];

    let mut intentos = 0;
    let max_intentos = palabra_secreta.len();

    loop {
        // Mostrar el estado actual del juego
        println!("La palabra hasta el momento es: {}", letras_adivinadas.iter().collect::<String>());
        println!("Adivinaste las siguientes letras: {}", letras_ingresadas_adivinadas);
        println!("Te quedan {} intentos", max_intentos - intentos);

        // leo de la pantalla
        let mut letra = String::new();
        print!("Ingresa una letra: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut letra).expect("Error al leer la pantalla del usuario");
        let letra = letra.trim().chars().next().expect("No se ingresó ninguna letra");

        // check si la letra está en la palabra secreta
        let mut acierto = false;
        for (i, c) in palabra_secreta.chars().enumerate() {
            if c == letra {
                letras_adivinadas[i] = letra;
                acierto = true;
            }
        }

        // Verificar si el jugador ganó o perdió
        if letras_adivinadas.iter().all(|&x| x != '_') {
            println!("¡Ganaste!");
            println!("la palabra era {}",palabra_secreta);
            break;
        }
        if acierto{
            if ! letras_ingresadas_adivinadas.is_empty() {
                letras_ingresadas_adivinadas.push('-');
            }
            letras_ingresadas_adivinadas.push(letra);
        }else {
            intentos += 1;
            println!("Esa letra no está en la palabra secreta");
        }

        if intentos == max_intentos {
            println!("¡Perdiste! La palabra secreta era: {}", palabra_secreta);
            break;
        }
    }
}
