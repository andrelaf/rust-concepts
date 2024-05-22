
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let inteiro: i32 = 10;
    println!("Valor da variável inteiro: {}, {}", inteiro, type_of(inteiro));

    let int_to_float: f32 = inteiro as f32;
    println!("Valor da variável inteiro convertido para float: {}, {}", int_to_float, type_of(int_to_float));

    let float: f32 = 2.5;
    println!("Valor da variável float: {}, {}", float, type_of(float));

    //O ponto flutuante é truncado
    let float_to_int: i32 = float as i32;
    println!("Valor da variável float convertido para inteiro: {}, {}", float_to_int, type_of(float_to_int));

    //Necessário & para referenciar a variável, porque a string passa o ownership para a variável
    //como a propriedaed passa para função, ela existirá somente na função, e não no escopo principal
    //Dessa forma, quando tentar acessar a variável string_to_int, o compilador retornará um erro
    let int_to_string = inteiro.to_string();
    println!("Valor da variável inteiro convertido para string: {}, {}", int_to_string, type_of(&int_to_string));

    let string: &str = "42";

    let string_to_int = string.parse::<i64>().unwrap();
    println!("Valor da variável string convertido para inteiro: {}, {}", string_to_int, type_of(&string_to_int));

}
