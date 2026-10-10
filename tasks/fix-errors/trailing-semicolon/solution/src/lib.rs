pub struct Loan {
    pub principal: f64,
    pub yearly_rate: f64,
    pub years: u32,
}

impl Loan {
    /// Interest owed using simple (not compound) interest.
    pub fn simple_interest(&self) -> f64 {
        self.principal * self.yearly_rate * self.years as f64
    }

    pub fn total_owed(&self) -> f64 {
        self.principal + self.simple_interest()
    }
}
