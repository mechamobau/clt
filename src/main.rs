use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Subcommand)]
enum CommandEnum {
    Rescisao,
    Liquido,
    Ferias,
    Salario13,
    Fgts,
    HorasExtras,
    Beneficios,
    Simulacao,
    Contrato,
    Inss,
    Irrf,
    Holerite,
    Reajuste,
    Aviso,
    Jornada,
}

struct Salario {
    valor: f64,
    dependentes: u32,
    vale_transporte: f64,
    vale_refeicao: f64,
}

impl Salario {
    fn calcular_inss(&self) -> f64 {
        let salario = self.valor;
        if salario <= 1320.00 {
            salario * 0.075
        } else if salario <= 2571.29 {
            1320.00 * 0.075 + (salario - 1320.00) * 0.09
        } else if salario <= 3856.94 {
            1320.00 * 0.075 + (2571.29 - 1320.00) * 0.09 + (salario - 2571.29) * 0.12
        } else if salario <= 7507.49 {
            1320.00 * 0.075
                + (2571.29 - 1320.00) * 0.09
                + (3856.94 - 2571.29) * 0.12
                + (salario - 3856.94) * 0.14
        } else {
            1320.00 * 0.075
                + (2571.29 - 1320.00) * 0.09
                + (3856.94 - 2571.29) * 0.12
                + (7507.49 - 3856.94) * 0.14
        }
    }

    fn calcular_irrf(&self) -> f64 {
        let inss = self.calcular_inss();
        let deducao_dependente = 189.59 * self.dependentes as f64;
        let base_calculo = self.valor - inss - deducao_dependente;

        if base_calculo <= 1903.98 {
            0.0
        } else if base_calculo <= 2826.65 {
            (base_calculo * 0.075) - 142.80
        } else if base_calculo <= 3751.05 {
            (base_calculo * 0.15) - 354.80
        } else if base_calculo <= 4664.68 {
            (base_calculo * 0.225) - 636.13
        } else {
            (base_calculo * 0.275) - 869.36
        }
    }

    fn calcular_salario_liquido(&self) -> f64 {
        let inss = self.calcular_inss();
        let irrf = self.calcular_irrf();
        let vt = self.valor * 0.06;
        let vt_deducao = if vt < self.vale_transporte {
            vt
        } else {
            self.vale_transporte
        };
        let vr = self.vale_refeicao;
        self.valor - inss - irrf - vt_deducao - vr
    }
}

#[derive(Parser)]
#[command(name = "clt")]
#[command(about = "Ferramenta de linha de comando para cálculos trabalhistas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CommandEnum,
}

fn main() {
    let cli = Cli::parse();

    let valor: f64 = input("Digite o seu salário bruto (exemplo: 1640.00): ")
        .trim()
        .parse()
        .expect("Digite um número válido");

    let dependentes = input("Número de dependentes: ")
        .trim()
        .parse()
        .expect("Digite um número válido");

    let vale_refeicao = input("Valor de Vale Refeição: ")
        .trim()
        .parse()
        .expect("Digite um número válido");

    let vale_transporte = input("Valor de Vale Transporte: ")
        .trim()
        .parse()
        .expect("Digite um número válido");

    let salario = Salario {
        dependentes,
        vale_refeicao,
        vale_transporte,
        valor,
    };

    match &cli.command {
        CommandEnum::Rescisao => {
            println!("Calculadora de rescisão");
            // TODO.
        }

        CommandEnum::Liquido => {
            println!("Calculadora de salário líquido");
            println!("---------------------------");
            println!(
                "Salário líquido: R$ {:.2}\nIRRF: R$ {:.2} \nINSS: R$ {:.2}",
                salario.calcular_salario_liquido(),
                salario.calcular_irrf(),
                salario.calcular_inss(),
            );
        }

        CommandEnum::Ferias => {
            println!("Calculadora de férias");
            // TODO.
        }

        CommandEnum::Salario13 => {
            println!("Calculadora de 13º salário");
            // TODO.
        }

        CommandEnum::Fgts => {
            println!("Consulta e cálculo do FGTS");
            // TODO.
        }

        CommandEnum::HorasExtras => {
            println!("Calculadora de horas extras e adicional noturno");
            // TODO.
        }

        CommandEnum::Beneficios => {
            println!("Calculadora de benefícios");
            // TODO.
        }

        CommandEnum::Simulacao => {
            println!("Simulação de diferentes cenários de remuneração e benefícios");
            // TODO.
        }

        CommandEnum::Contrato => {
            println!("Verificação de detalhes do contrato de trabalho");
            // TODO.
        }

        CommandEnum::Inss => {
            println!("Calculadora de contribuição ao INSS");
            // TODO.
        }

        CommandEnum::Irrf => {
            println!("Calculadora de Imposto de Renda Retido na Fonte (IRRF)");
            // TODO.
        }

        CommandEnum::Holerite => {
            println!("Geração de holerites simulados");
            // TODO.
        }

        CommandEnum::Reajuste => {
            println!("Simulação de reajustes salariais");
            // TODO.
        }

        CommandEnum::Aviso => {
            println!("Calculadora de aviso prévio");
            // TODO.
        }

        CommandEnum::Jornada => {
            println!("Gerenciamento de jornada de trabalho");
            // TODO.
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    input
}
