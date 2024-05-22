use std::fs::File;
use std::io::{self, ErrorKind, Read};


fn main() {

    // catch_unwind is a function that allows you to catch a panic and handle it
    let result = std::panic::catch_unwind(|| {
      let a = with_panic(0);

      Ok::<i32, &str>(a)
    });

    match result {
      Ok(valor) => {
        println!("The function returned: {}", valor.unwrap())
      },
      Err(_)=>  {
        println!("The function panicked")
      }
    };

    // Para manter uma string literal utilize o prefixo r#""#
    let file_result = read_file(r"C:\repos\rust-concepts\errors\src\main.rs");

    match file_result {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File not found");
                },
                _ => {
                    println!("An error occurred: {:?}", error);
                }
            }
        }
    }

  let division_result = to_divide(10.0, 5.0);
  match division_result {
    Some(result) => {
      println!("The division result is: {}", result);
    },
    None => {
      println!("Division by zero is not allowed");
    }
  }
}

fn with_panic(valor: i32) -> i32 {
    if valor == 0 {
        panic!("Value is zero");
    }
                                                
    valor
}

fn read_file(path: &str) -> Result<String, io::Error> {

   // Caso aconteça um error ao tentar abrir o arquivo, o erro será retornado devido a utilização do operador ?.
    let mut file = File::open(path)?;

    let mut content = String::new();

    file.read_to_string(&mut content);

    Ok(content)
}

fn to_divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
      None
    }else{
      Some(dividend / divisor)
    }
}