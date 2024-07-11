use clap::{Parser, Subcommand};
use core::Salario;
use utils::input;

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
    Reajuste,
    Aviso,
    Jornada,
}

mod core;
mod utils;

#[derive(Parser)]
#[command(name = "clt")]
#[command(about = "Ferramenta de linha de comando para cálculos trabalhistas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CommandEnum,
}

fn main() {
    let cli = Cli::parse();

    let bruto: f64 = input("Digite o seu salário bruto (exemplo: 1640.00): ")
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
        bruto,
    };

    match &cli.command {
        CommandEnum::Rescisao => {
            println!("Calculadora de rescisão");
            println!("---------------------------");

            let meses_trabalhados = input("Meses trabalhados: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Valor da Rescisão R$ {:.2}",
                salario.calcular_rescisao(meses_trabalhados)
            )
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
            println!("---------------------------");

            println!("Valor das Férias: R$ {:.2}", salario.calcular_ferias())
        }

        CommandEnum::Salario13 => {
            println!("Calculadora de 13º salário");
            println!("---------------------------");

            let meses_trabalhados = input("Meses trabalhados: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Valor do 13º salário: R$ {:.2}",
                salario.calcular_13(meses_trabalhados)
            )
        }

        CommandEnum::Fgts => {
            println!("Consulta e cálculo do FGTS");
            println!("---------------------------");

            println!("Valor do FGTS: R$ {:.2}", salario.calcular_fgts())
        }

        CommandEnum::HorasExtras => {
            println!("Calculadora de horas extras e adicional noturno");
            println!("---------------------------");

            let horas = input("Horas trabalhadas: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            let percentual = input("Percentual: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Valor de Horas Extras: R$ {:.2}",
                salario.calcular_horas_extras(horas, percentual)
            )
        }

        CommandEnum::Beneficios => {
            println!("Calculadora de benefícios");
            println!("---------------------------");

            println!("Benefícios: R$ {:.2}", salario.calcular_beneficios())
        }

        CommandEnum::Simulacao => {
            println!("Simulação de diferentes cenários de remuneração e benefícios");
            println!("---------------------------");

            println!("Simulação: R$ {:.2}", salario.simulacao())
        }

        CommandEnum::Contrato => {
            println!("Verificação de detalhes do contrato de trabalho");
            println!("---------------------------");

            let meses_trabalhados = input("Meses trabalhados: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Contrato: R$ {:.2}",
                salario.calcular_contrato(meses_trabalhados)
            );
        }

        CommandEnum::Inss => {
            println!("Calculadora de contribuição ao INSS");
            println!("---------------------------");

            println!("Valor do INSS: R$ {:.2}", salario.calcular_inss());
        }

        CommandEnum::Irrf => {
            println!("Calculadora de Imposto de Renda Retido na Fonte (IRRF)");
            println!("---------------------------");

            println!("Valor do IRRF: R$ {:.2}", salario.calcular_irrf());
        }

        CommandEnum::Reajuste => {
            println!("Simulação de reajustes salariais");
            println!("---------------------------");

            let percentual = input("Percentual: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Valor de Reajuste: R$ {:.2}",
                salario.calcular_reajuste(percentual)
            );
        }

        CommandEnum::Aviso => {
            println!("Calculadora de aviso prévio");
            println!("---------------------------");

            println!("Valor do Aviso: R$ {:.2}", salario.calcular_aviso());
        }

        CommandEnum::Jornada => {
            println!("Gerenciamento de jornada de trabalho");
            println!("---------------------------");

            let horas = input("Horas trabalhadas: ")
                .trim()
                .parse()
                .expect("Digite um número válido");

            println!(
                "Valor da Jornada: R$ {:.2}",
                salario.calcular_jornada(horas)
            );
        }
    }
}
