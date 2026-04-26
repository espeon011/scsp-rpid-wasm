use scsp_rpid_wasm::{model, scsp};

const INPUT: &str = "MALSYCPKGT
MQSSLNAIPV
MPLSYQHFRK
MEEHVNELHD
MSNFDAIRAL
MFRNQNSRNG
MFYAHAFGGY
MSKFTRRPYQ
MSFVAGVTAQ
MESLVPGFNE";

fn main() {
    let instance: scsp::ScspInstance<char> = INPUT.parse().unwrap();
    let model = model::ModelRpid::new(&instance);
    let solution = model.solve(10);

    println!();
    println!("Objective: {}", solution.objective().unwrap());
    println!("Bound: {}", solution.bound.unwrap());
}
