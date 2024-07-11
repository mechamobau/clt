# `clt`

Esta é uma ferrament cli para CLT (Consolidação das Leis do Trabalho) escrita em Rust, projetada para calcular diversas componentes relacionadas ao salário de um empregado, incluindo salário líquido, rescisão, férias, 13º salário, FGTS, horas extras, benefícios, simulação de salário, contrato, INSS, IRRF, holerite, reajuste, aviso prévio e jornada de trabalho.

## Funcionalidades

- Cálculo do salário líquido considerando INSS e IRRF
- Cálculo de rescisão
- Cálculo de férias
- Cálculo de 13º salário
- Cálculo de FGTS
- Cálculo de horas extras
- Cálculo de benefícios
- Simulação de salário
- Cálculo de contrato
- Cálculo de reajuste salarial
- Cálculo de aviso prévio
- Cálculo de jornada de trabalho

## Instalação

1. Certifique-se de ter o [Rust](https://www.rust-lang.org/) instalado em sua máquina.
2. Clone este repositório:
   ```bash
   git clone https://github.com/mechamobau/clt.git
   ```
3. Navegue até o diretório do projeto:
   ```bash
   cd clt
   ```
4. Compile o projeto:
   ```bash
   cargo build --release
   ```

## Uso

### Comandos Disponíveis

- `liquido`: Calcula o salário líquido.
- `rescisao`: Calcula a rescisão.
- `ferias`: Calcula as férias.
- `13`: Calcula o 13º salário.
- `fgts`: Calcula o FGTS.
- `horas-extras`: Calcula as horas extras.
- `beneficios`: Calcula os benefícios.
- `simulacao`: Simula o salário líquido.
- `contrato`: Calcula o valor do contrato.
- `reajuste`: Reajusta o salário.
- `aviso`: Calcula o aviso prévio.
- `jornada`: Calcula o salário por jornada de trabalho.

### Exemplos de Uso

#### Calcular Salário Líquido

```bash
clt liquido --bruto 5000 --dependentes 1
```

#### Calcular Rescisão

```bash
clt rescisao --bruto 5000 --dependentes 1 --meses-trabalhados 12
```

## Estrutura do Código

A estrutura principal `Salario` armazena as informações necessárias para realizar os cálculos.

```rust
struct Salario {
    bruto: f64,
    dependentes: u32,
    vale_transporte: f64,
    vale_refeicao: f64,
}
```

## Métodos Implementados

- `calcular_inss(&self) -> f64`: Calcula a contribuição ao INSS.
- `calcular_irrf(&self) -> f64`: Calcula o Imposto de Renda Retido na Fonte (IRRF).
- `calcular_liquido(&self) -> f64`: Calcula o salário líquido.
- `calcular_rescisao(&self, meses_trabalhados: u32) -> f64`: Calcula a rescisão.
- `calcular_ferias(&self) -> f64`: Calcula o valor das férias.
- `calcular_13(&self, meses_trabalhados: u32) -> f64`: Calcula o 13º salário.
- `calcular_fgts(&self) -> f64`: Calcula o FGTS.
- `calcular_horas_extras(&self, horas: f64, percentual: f64) -> f64`: Calcula as horas extras.
- `calcular_beneficios(&self) -> f64`: Calcula os benefícios.
- `simulacao(&self) -> f64`: Simula o salário líquido.
- `calcular_contrato(&self, meses: u32) -> f64`: Calcula o valor do contrato.
- `calcular_reajuste(&mut self, percentual: f64)`: Reajusta o salário.
- `calcular_aviso(&self) -> f64`: Calcula o aviso prévio.
- `calcular_jornada(&self, horas: f64) -> f64`: Calcula o salário por jornada de trabalho.

## Contribuição

1. Fork este repositório.
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`).
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`).
4. Faça push para a branch (`git push origin feature/nova-feature`).
5. Abra um Pull Request.

## Licença

Este projeto está licenciado sob a licença MIT
