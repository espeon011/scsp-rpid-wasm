use scsp_rpid_wasm::{model, scsp};

const INPUT: &str = "ATGGGATACG
ATACCTTCCC
CACGAATTGA
TAAAATCTGT
AGGTAACAAA
TTCCTAGGTA
TTGTAGATCT
TGGGAAGTTC
TTCCACAACT
TCTAAACGAA";

fn main() {
    let instance: scsp::ScspInstance<char> = INPUT.parse().unwrap();
    let model = model::ModelRpid::new(&instance);
    let solution = model.solve(10);

    println!();
    println!("Objective: {}", solution.objective().unwrap());
    println!("Bound: {}", solution.bound.unwrap());
}
