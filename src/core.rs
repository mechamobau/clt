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
