use alloc::string::String;

#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub program: String,
    pub prover: String,
    pub cycles: usize,
    pub overhead_duration: f64,
    pub core_prove_duration: f64,
    pub core_verify_duration: f64,
    pub core_proof_size: usize,
}
