mod model;
mod scsp;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ComputeResult {
    solution_inner: String,
    dual_bound_inner: Option<f32>,
}

#[wasm_bindgen]
impl ComputeResult {
    #[wasm_bindgen(getter)]
    pub fn solution(&self) -> String {
        self.solution_inner.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn dual_bound(&self) -> f32 {
        self.dual_bound_inner.unwrap_or(f32::NAN)
    }

    fn from_solution(sol: scsp::ScspSolution<char>) -> Self {
        if let (Some(seq), Some(bound)) = (sol.seq, sol.bound) {
            ComputeResult {
                solution_inner: seq.iter().collect(),
                dual_bound_inner: Some(bound as f32),
            }
        } else {
            ComputeResult {
                solution_inner: String::from(""),
                dual_bound_inner: None,
            }
        }
    }
}

#[wasm_bindgen]
pub fn superseq(input: &str) -> ComputeResult {
    let instance = scsp::ScspInstance::from_str(input);
    let model = model::ModelRpid::new(&instance);
    let solution = model.solve(1u32);
    ComputeResult::from_solution(solution)
}
