# rust-concepts

Aqui estão alguns comandos úteis para trabalhar com Rust:

- `rustc main.rs`: Compila o arquivo `main.rs` usando o compilador Rust (`rustc`).
- `cargo new nome-projeto`: Cria um novo projeto Rust com o nome especificado em `nome-projeto` usando o gerenciador de pacotes Cargo (`cargo`).
- `cargo run`: Compila e executa o projeto atual usando o Cargo.
- `cargo run --release`: Compila e executa o projeto atual em modo de lançamento, otimizando o desempenho.
- `cargo init`: Inicializa um novo projeto Rust vazio no diretório atual usando o Cargo.

Certifique-se de ter o Rust e o Cargo instalados em seu sistema antes de executar esses comandos. Para mais informações sobre como começar com Rust, consulte a documentação oficial em [https://www.rust-lang.org/](https://www.rust-lang.org/).


## Tipos de Dados
Não possui null.
https://doc.rust-lang.org/reference/types.html

## Tipos Escalares
Integers
 - Signed - Posso ter valores negativos
 - Unsigned - Não posso ter valores negativos
Floating Point
Numbers
Booleans
Character - Rust utiliza unicode.


## Tipos Compostos:
Tuplas - Posos ter vários tipos, comprimento fixo.
Arrays - Precisa ser do mesmo tipo, comprimento fixo.


## Ownership
Cada valor tem exatamente um "proprietário". Quando proprietário sai de escopo
o valor automaticamente e liberado da memoria.

```Rust
let texto = String::from("hello");
let texto2 = texto; // Aqui transferimos a responsabilidade e texto será limpada.
```

## Borrow
Para referênciar valores sem transferir a propriedade, Rust
utiliza referências. Elas podem ser mutáveis ou imutáveis. 

```Rust
let mut valor = 50;
let referencia = &valor; // Por padrão essa referência não é mutavel.
let referencia_mut = &mut valor; //Nesse caso você consegue alterar o "valor"
```

Como o tipo int e menor que string, o rust poderia copiar sem precisar da referência,
devido o custo de memoria para fazer essa alocação esse tipo int e menos custoso que a string.
É mais rapido copiar e alocar, do que ter todo controle de mudar o proprietário e destruir.

## Fuctions

Retorno
Em funções rust você pode retornar de uma função de duas maneiras, 
explicito utilizando `return` mais `;`.

Retorno implícito, a ultima instrução sem `;` e sem a palavra `return`.

Se uma função não retorna nada, esse tipo é indicado como (), não existe null.

Todo bloco `{}` é uma expressão, e todo expressão e passível de colocar em uma variável.
