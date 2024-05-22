fn main() {
    se();
    declaracao_if();
    loop_infinite();
    while_finite();
    for_iterator();
    match_statement();
}


fn se(){

    let number: i32 = 99;

    if number > 0 {
        println!("O número {} é positivo", number);
    } else if  number == 0 {
        println!("O número é zero");
    } else {
        println!("O número {} é negativo", number);
    }
}


// Todo escopo retorna valor incluindo o if
fn declaracao_if() {
    let number = 3;

    let result = if number > 0 {
        "positivo"
    } else {
        "negativo"
    };

    // O ponto e vírgula é necessário porque é uma expressão quando usa dessa forma
    // aqui é o que seria mais proximo de um if ternário em outras linguagens
    println!("O número {} é {}", number, result);
}


fn loop_infinite(){
    let mut i = 0;
    loop {
        println!("i = {}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }
}

fn while_finite(){
    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }
}

fn for_iterator(){

    // Sem o = ele vai até 9
   for i in 0..=10{
       println!("i = {}", i);
   }

    let vetor = vec![1, 2, 3, 4, 5];
    for i in vetor {
        println!("i = {}", i);
    }

    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("i = {}", i);
    }
}

fn match_statement(){
    let number = 3;

    match number {
        1 => println!("O número é 1"),
        2 => println!("O número é 2"),
        3 => println!("O número é 3"),
        _ => println!("O número é diferente de 1, 2 e 3"),
    }
}