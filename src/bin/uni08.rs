use scsp_rpid_wasm::{model, scsp};

const INPUT: &str = "tkgnkuhmpxnhtqgxzvxis
iojiqfolnbxxcvsuqpvissbxf
ulcinycosovozpplp
igevazgbrddbcsvrvnngf
pyplrzxucpmqvgtdfuivcdsbo
pbdevdcvdpfzsmsbroqvbbh
enbczfjtvxerzbrvigple
rxwxqkrdrlctodtmprpxwd";

fn main() {
    let instance: scsp::ScspInstance<char> = INPUT.parse().unwrap();
    let model = model::ModelRpid::new(&instance);
    let solution = model.solve(10);

    println!();
    println!("Objective: {}", solution.objective().unwrap());
    println!("Bound: {}", solution.bound.unwrap());
}
