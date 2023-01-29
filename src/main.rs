use std::io::{self, Write};

fn divide_panic(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("División entre 0")
    }
    a / b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Division entre 0"));
    }
    Ok(a / b)
}

fn print_result(a: f64, b: f64, res: f64) {
    println!("Resultado: {} / {} = {}", a, b, res);
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la opción");
    input.pop();
    match input.parse::<f64>() {
        Ok(number) => number,
        Err(_) => read_int() as f64,
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la opción");
    input.pop();
    input.parse::<i32>().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Impresión del menú inicial
    println!("Errores en Rust\n");
    println!("Este programa es para ilustrar los diferentes tipos de errores en rust y su manejo.\nSeleccione un tipo de error y después ingrese dos números a dividir\n");
    println!("Tipos de Errores:");
    println!("1) Panic");
    println!("2) Result - Match");
    println!("3) Result - Unwrap");
    println!("4) Result - Expect");
    println!("5) Result - Propagate");
    print!("Opción > ");
    io::stdout().flush()?;

    // Lectura de la opción de manejo de errores seleccionada
    let option = read_int();
    if option > 5 {
        return Err("Opción inválida")?;
    }

    // Lectura de los dos números a dividir, notese que se utiliza manejo
    // de errores incluso al leer y transformar las entradas por stdio
    print!("a: ");
    io::stdout().flush()?;
    let a = read_number();
    print!("b: ");
    io::stdout().flush()?;
    let b = read_number();

    // Código de ejemplo de los distintos tipos de manejo de errores
    match option {
        1 => {
            let res = divide_panic(a, b);
            print_result(a, b, res);
        }
        2 => {
            let divide_result = divide(a, b);
            match divide_result {
                Ok(res) => print_result(a, b, res),
                Err(e) => println!("Match error: {}", e),
            }
        }
        3 => {
            let res = divide(a, b).unwrap();
            print_result(a, b, res);
        }
        4 => {
            let res = divide(a, b).expect("Expect error");
            print_result(a, b, res);
        }
        5 => {
            let res = divide(a, b)?;
            print_result(a, b, res);
        }
        _ => {}
    };
    Ok(())
}
