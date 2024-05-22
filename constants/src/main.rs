fn main() {

    // Necessário sempre definir o tipo em constant, não consegue inferir o valor
    // Sempre que usa a contant ela será substituída pelo valor
    // No binário final não terá a constante, mas sim o valor
    const NAME_COMPANY: &str = "Company";
    println("Valor da constant é: {}", NAME_COMPANY);

    // Por definição, constantes são imutáveis
    let name_company: &str = "Name";
    println("Valor da variável é: {}", NAME_COMPANY);
  
}
