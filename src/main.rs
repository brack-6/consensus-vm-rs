use rand::prelude::*;

/// A hyper-edge: maps N input registers through a nonlinear kernel to one output.
/// This is simultaneously a VM instruction and a consensus update rule.
#[derive(Clone, Debug)]
struct Edge {
    op: OpKind,
    inputs: Vec<usize>,
    output: usize,
}

#[derive(Clone, Debug)]
enum OpKind {
    Sum,   // output = tanh(sum of inputs)
    Prod,  // output = tanh(product of inputs)
    Max,   // output = tanh(max of inputs)
    Mean,  // output = tanh(mean of inputs)
}

impl Edge {
    fn eval(&self, regs: &[f64]) -> f64 {
        let vals: Vec<f64> = self.inputs.iter().map(|&i| regs[i]).collect();
        let raw: f64 = match self.op {
            OpKind::Sum  => vals.iter().sum(),
            OpKind::Prod => vals.iter().product(),
            OpKind::Max  => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            OpKind::Mean => vals.iter().sum::<f64>() / vals.len() as f64,
        };
        raw.tanh()
    }
}

/// One consensus step: apply all edges in parallel (read old state, write new state).
fn consensus_step(edges: &[Edge], regs: &mut Vec<f64>) {
    let old = regs.clone();
    for e in edges {
        regs[e.output] = e.eval(&old);
    }
}

/// Stochastic rewiring: add or remove edges with probability p.
/// This is the "temporal hypergraph" dynamics from Source A.
fn rewire(edges: &mut Vec<Edge>, n_regs: usize, rng: &mut impl Rng, p: f64) {
    // possibly remove a random edge
    if !edges.is_empty() && rng.gen_bool(p) {
        let idx = rng.gen_range(0..edges.len());
        edges.swap_remove(idx);
    }
    // possibly add a new random edge
    if rng.gen_bool(p) {
        let arity = rng.gen_range(2..=4);
        let inputs = (0..arity).map(|_| rng.gen_range(0..n_regs)).collect();
        let output = rng.gen_range(0..n_regs);
        let op = match rng.gen_range(0..4) {
            0 => OpKind::Sum,
            1 => OpKind::Prod,
            2 => OpKind::Max,
            _ => OpKind::Mean,
        };
        edges.push(Edge { op, inputs, output });
    }
}

/// Measure convergence: mean absolute change in registers between steps.
fn delta(old: &[f64], new: &[f64]) -> f64 {
    old.iter().zip(new.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>() / old.len() as f64
}

fn main() {
    let mut rng = thread_rng();
    let n_regs = 8;
    let steps  = 2000;
    let mut_rate = 0.05;

    // Random initial register state
    let mut regs: Vec<f64> = (0..n_regs)
        .map(|_| rng.gen_range(-1.0..1.0))
        .collect();

    // Initial program hypergraph — random edges
    let mut edges: Vec<Edge> = (0..12).map(|_| {
        let arity = rng.gen_range(2..=3);
        Edge {
            op: match rng.gen_range(0..4) {
                0 => OpKind::Sum, 1 => OpKind::Prod,
                2 => OpKind::Max, _ => OpKind::Mean,
            },
            inputs: (0..arity).map(|_| rng.gen_range(0..n_regs)).collect(),
            output: rng.gen_range(0..n_regs),
        }
    }).collect();

    println!("t     edges  delta    regs");
    println!("{:-<60}", "");

    for t in 0..=steps {
        let old = regs.clone();
        consensus_step(&edges, &mut regs);
        rewire(&mut edges, n_regs, &mut rng, mut_rate);
        let d = delta(&old, &regs);

        if t % 200 == 0 {
            let reg_str: Vec<String> = regs.iter()
                .map(|r| format!("{:+.3}", r))
                .collect();
            println!("{:<5} {:<6} {:.6}  [{}]",
                t, edges.len(), d, reg_str.join(", "));
        }
    }

    println!("{:-<60}", "");
    println!("Final register state:");
    for (i, r) in regs.iter().enumerate() {
        println!("  reg[{}] = {:+.6}", i, r);
    }
    println!("Final edge count: {}", edges.len());
}
