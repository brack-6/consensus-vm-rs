# consensus-vm-rs

A virtual machine whose instruction dispatch loop is a nonlinear consensus update on a temporal hypergraph.

**Dan Baker** — Independent Research, Bogotá, Colombia

## The Isomorphism

A VM instruction is a hyperedge: it maps N input registers through a nonlinear kernel to one output register. A VM execution trace is therefore a walk on a hypergraph. This is the same mathematical object as nonlinear consensus dynamics on a temporal network.

| VM concept | Consensus dynamics |
|------------|-------------------|
| Register | Network node |
| Instruction (hyperedge) | Group interaction |
| Execution step | Consensus update |
| Control flow mutation | Temporal rewiring |
| Convergence | Network attractor |

Both are: **nonlinear dynamics on a temporally evolving hypergraph.**

## What it does

- State vector of N floating-point registers (network nodes)
- Program is a set of hyperedges, each mapping 2-4 inputs to one output via a nonlinear kernel (tanh ∘ {sum, product, max, mean})
- Each step applies all edges in parallel (true consensus update)
- Stochastic rewiring adds/removes edges after each step (temporal hypergraph dynamics)
- System finds attractors, gets perturbed, finds new ones

## Result
t     edges  delta    regs
0     12     0.546844  [-0.366, -0.238, +0.215, +0.746, ...]
200   4      0.000000  [-0.366, +0.181, -0.000, +0.281, ...]
800   2      0.000928  [+0.338, +0.000, +0.326, +0.000, ...]
1400  7      0.000001  [+0.759, +0.702, +0.888, +0.752, ...]
2000  2      0.000000  [+0.383, +0.622, +0.000, +0.614, ...]

Edge count self-simplifies over time (12→2). The system finds an attractor, rewiring destabilizes it, a new attractor forms. Disconnected registers converge to zero.

## Run

```bash
cargo run --release
```

## Paper

Baker, D. (2026). Consensus-VM: A Virtual Machine as Nonlinear Consensus Dynamics on a Temporal Hypergraph. Zenodo. *(forthcoming)*

## Related

- [ZK-GSP](https://github.com/brack-6/zk-gsp) — ZK proofs of GNN inference
- [LeapGNN](https://github.com/brack-6/LeapGNN) — Symplectic integration as GNN message passing

## Donate

[PayPal](https://paypal.me/bakermoto)
