pub struct Salario {
    pub bruto: f64,
    pub dependentes: u32,
    pub vale_transporte: f64,
    pub vale_refeicao: f64,
}

impl Salario {
    pub fn calcular_inss(&self) -> f64 {
        let salario = self.bruto;
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

    pub fn calcular_irrf(&self) -> f64 {
        let inss = self.calcular_inss();
        let deducao_dependente = 189.59 * self.dependentes as f64;
        let base_calculo = self.bruto - inss - deducao_dependente;

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

    pub fn calcular_salario_liquido(&self) -> f64 {
        let inss = self.calcular_inss();
        let irrf = self.calcular_irrf();
        let vt = self.bruto * 0.06;
        let vt_deducao = if vt < self.vale_transporte {
            vt
        } else {
            self.vale_transporte
        };
        let vr = self.vale_refeicao;
        self.bruto - inss - irrf - vt_deducao - vr
    }

    pub fn calcular_rescisao(&self, meses_trabalhados: u32) -> f64 {
        let salario_liquido = self.calcular_salario_liquido();
        let aviso_previo = if meses_trabalhados > 12 {
            salario_liquido
        } else {
            salario_liquido / 2.0
        };
        let fgts = self.bruto * 0.08 * meses_trabalhados as f64;
        salario_liquido + aviso_previo + fgts
    }

    pub fn calcular_ferias(&self) -> f64 {
        self.bruto * 1.3333
    }

    pub fn calcular_13(&self, meses_trabalhados: u32) -> f64 {
        (self.bruto / 12.0) * meses_trabalhados as f64
    }

    pub fn calcular_13_parcelas(&self, meses_trabalhados: u32) -> (f64, f64) {
        let inss = self.calcular_inss();
        let irrf = self.calcular_irrf();
        let descontos = inss + irrf;

        let parcela_unica = self.calcular_13(meses_trabalhados);
        let primeira_parcela = parcela_unica / 2.0;
        let segunda_parcela = primeira_parcela - descontos;
        (primeira_parcela, segunda_parcela)
    }

    pub fn calcular_fgts(&self) -> f64 {
        self.bruto * 0.08
    }

    pub fn calcular_horas_extras(&self, horas: f64, percentual: f64) -> f64 {
        let valor_hora = self.bruto / 220.0;
        valor_hora * horas * (1.0 + percentual / 100.0)
    }

    pub fn calcular_beneficios(&self) -> f64 {
        self.vale_transporte + self.vale_refeicao
    }

    pub fn simulacao(&self) -> f64 {
        self.calcular_salario_liquido()
    }

    pub fn calcular_contrato(&self, meses: u32) -> f64 {
        self.calcular_salario_liquido() * meses as f64
    }

    pub fn calcular_reajuste(self, percentual: f64) -> f64 {
        self.bruto + self.bruto * percentual / 100.0
    }

    pub fn calcular_aviso(&self) -> f64 {
        self.bruto / 30.0 * 30.0
    }

    pub fn calcular_jornada(&self, horas: f64) -> f64 {
        self.bruto / 220.0 * horas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_inss() {
        let salario = Salario {
            bruto: 1500.00,
            dependentes: 0,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(
            salario.calcular_inss(),
            1320.00 * 0.075 + (1500.00 - 1320.00) * 0.09
        );
    }

    #[test]
    fn test_calcular_irrf() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(
            salario.calcular_irrf(),
            (3000.00 - salario.calcular_inss() - 189.59) * 0.075 - 142.80
        );
    }

    #[test]
    fn test_calcular_salario_liquido() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let inss = salario.calcular_inss();
        let irrf = salario.calcular_irrf();
        let vt_deducao = 100.00; // Assume que o vale transporte é menor que 6% do bruto
        assert_eq!(
            salario.calcular_salario_liquido(),
            3000.00 - inss - irrf - vt_deducao - 200.00
        );
    }

    #[test]
    fn test_calcular_rescisao() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let meses_trabalhados = 15;
        let salario_liquido = salario.calcular_salario_liquido();
        let aviso_previo = salario_liquido;
        let fgts = 3000.00 * 0.08 * meses_trabalhados as f64;
        assert_eq!(
            salario.calcular_rescisao(meses_trabalhados),
            salario_liquido + aviso_previo + fgts
        );
    }

    #[test]
    fn test_calcular_ferias() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(salario.calcular_ferias(), 3000.00 * 1.3333);
    }

    #[test]
    fn test_calcular_13() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let meses_trabalhados = 12;
        assert_eq!(salario.calcular_13(meses_trabalhados), 3000.00);
    }

    #[test]
    fn test_calcular_13_parcelas() {
        let salario = Salario {
            bruto: 2000.00,
            dependentes: 1,
            vale_transporte: 0.0,
            vale_refeicao: 0.0,
        };

        let meses_trabalhados = 12;
        assert_eq!(
            salario.calcular_13_parcelas(meses_trabalhados),
            (1000.00, 839.8)
        )
    }

    #[test]
    fn test_calcular_fgts() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(salario.calcular_fgts(), 3000.00 * 0.08);
    }

    #[test]
    fn test_calcular_horas_extras() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let horas = 10.0;
        let percentual = 50.0;
        assert_eq!(
            salario.calcular_horas_extras(horas, percentual),
            (3000.00 / 220.0) * horas * 1.5
        );
    }

    #[test]
    fn test_calcular_beneficios() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(salario.calcular_beneficios(), 100.00 + 200.00);
    }

    #[test]
    fn test_simulacao() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(salario.simulacao(), salario.calcular_salario_liquido());
    }

    #[test]
    fn test_calcular_contrato() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let meses = 12;
        assert_eq!(
            salario.calcular_contrato(meses),
            salario.calcular_salario_liquido() * meses as f64
        );
    }

    #[test]
    fn test_calcular_reajuste() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let percentual = 10.0;
        assert_eq!(
            salario.calcular_reajuste(percentual),
            3000.00 + 3000.00 * percentual / 100.0
        );
    }

    #[test]
    fn test_calcular_aviso() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        assert_eq!(salario.calcular_aviso(), 3000.00);
    }

    #[test]
    fn test_calcular_jornada() {
        let salario = Salario {
            bruto: 3000.00,
            dependentes: 1,
            vale_transporte: 100.00,
            vale_refeicao: 200.00,
        };
        let horas = 8.0;
        assert_eq!(salario.calcular_jornada(horas), (3000.00 / 220.0) * horas);
    }
}
