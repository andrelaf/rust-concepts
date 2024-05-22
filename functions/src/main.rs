fn main() {
    nome_da_funcao();

    let a = com_retorno();
    println!("a = {}", a);

    let b = sem_retorno();
    println!("b = {}", b);

    let c = maior_valor(3, 5);
    println!("c = {}", c);


    let d: i32 = 5;
    let resultado_incrementado = incrementar(d);
    println!("resultado_incrementado = {}", resultado_incrementado);

}

fn nome_da_funcao() {
    println!("Hello, world!");
}

fn com_retorno() -> i8 {
    return 3;
}

fn sem_retorno() -> i8 {
    3
}

//o bloco é opcional, mas é uma boa prática colocar para melhorar a legibilidade
fn maior_valor(a: i32, b: i32) -> i32 {
    {
        if a > b {
            return a;
        } else {
            return b;
        }
    }   
}

fn incrementar(mut a: i32) -> i32 {
    a += 1;
    a
}