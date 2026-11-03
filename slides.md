---
theme: default
title: Formally Verifying Software with Agents
fonts:
  sans: Inter
transition: slide-left
layout: cover
class: text-center
---

# Formally Verifying Software with Agents
<!-- (authored by agents unless marked 🧑) -->

<div class="text-left inline-block">

- What's the point
- Summer: what agents can(not) do
- What changed since
- Next: distributed liveness

</div>

<!--
Opening: "These are May-July runs on models that are outdated now, and I haven't revisited the details since, so I'll focus on what we learned rather than exact numbers."
-->

---

- Agents: much faster development
    - but weaker correctness
- Formal verification: correctness
    - but too much work & slow
- Spend agent velocity on verification
    - get both


---

# Verus

- Macro + checker for Rust
- Code + spec + checked proof
- Unified

---

# Verification is too expensive

- IronFleet: ~3.7 person-years
    - method + 2 systems
    - proof ~8× the code

<!-- IronFleet SOSP 2015: 39,253 proof-related lines vs 5,114 executable lines (via search summary). Example: backup "Verus example". -->

---

# Summer: staged experiments

- Cut holes in 8 existing Verus projects
- Agent fill in the blank; verifier checks
- Cut proof → spec → impl

<!-- Projects: Vest (parser), Anvil (Kubernetes controllers), verified allocator. Hidden original used only for scoring. -->

---

# Verus projects used

<div class="small">

- Vest: binary parser and serializer combinators
- Node Replication: replicates a data structure across CPU sockets
- Anvil: Kubernetes controllers, including liveness
- Verified Storage: persistent-memory storage
- Memory Allocator: port of the mimalloc allocator
- IronKV: sharded key-value store
- Atmosphere: OS kernel
- NRKernel: OS page-table management

</div>

<!-- 8 projects from VeruSAGE-Bench. -->

---

# Finding 1: proofs mostly solved

- 143/167 proofs passed
    - when told where
- Failures: chasing local errors
- Sonnet 4.6

---

# Finding 2: specs are not

- Told which is missing: 18/18 filled
- Only code: restates code (useless)
- From English: how precise?
- gpt-5.5

---

# Finding 3: "verified" ≠ correct

- Weak spec still verifies
- No automatic way to check spec
- Spec is bottleneck

<!-- Example: backup "Weak spec". From-English builds: tested Rust + separately proved Verus model; no proof the Rust matches the model. -->

---

# The field since

- LLMs even better at proofs
- Got much attention
- Mainly single-threaded
- Unclear how to check spec

---

- Anthropic: 123 libraries never crash
    - free spec: "no crash"
    - single-threaded
- TLA-Verus: liveness on TLA+ models
    - Sonnet-5 got 7/10
    - no public impl

<!-- Anthropic spec came from fuzz tests. Model names and quotes: backup "Sources". TLA-Verus tasks are unnamed, unreleased GitHub specs. -->

---

- SWE-Proof: checking specs
    - impl + proof for spec
    - "mutants" (unit tests)
    - reject wrong spec by example
    - spec correctness: 85% → ~95%

<!-- Ma, Mikek et al., arXiv 2609.21190 (Berkeley, Georgia Tech, UIUC, AWS); Claude Opus 4.8; Python via Nagini/Lean. Self-written spec 85.6/85.0/83.8% vs 85.0% baseline; correct spec 96.2/94.0/95.2%. Spec synthesis alone passes the five-property audit only 46.0-61.4%. Tests miss defects: 26.8% of test-passing patches overturned by audit. -->

---

# Early reruns: Sonnet 5.5

- Proofs: allocator 87/87
    - controller 49/49
    - summer: stuck; 25/49
- Specs: 15/15 pass verifier
    - 1 wrong: forgot to remove a key

<!--
Allocator: full project "744 verified, 0 errors"; targets "closed: 87/87". Independent review found no spec weakening or shortcuts. Not a controlled model comparison: CLI and setup differ from summer.
Anvil controller: 49/49; original build.sh "126 verified, 0 errors (partial verification with --verify-*)"; same scope as summer Sonnet 4.6 25/49; run was interrupted and resumed. Caveat for both: agent added sound transparent spec helpers beyond what the prompt allowed (controller 3, allocator 1).
Specs: 15 named-target attempts (7 Anvil "398 verified, 0 errors", 8 Vest "527 verified, 0 errors"); one Anvil spec omitted `scheduled_reconciles.remove(cr_key)`, and a separate proof shows it differs from the original. Verifier acceptance is not a spec score.
-->

---

# Agents still cannot

- Guarantee correct spec
- Prove impl code
    - liveness proofs only on models
- Tackle real systems
    - tasks small, pre-cut, textbook

<!-- Spec: Sonnet 5.5 rerun above; SWE-Proof: agent's own spec gives no gain. Running code: TLA-Verus "Connecting implementations to the specifications via refinement proofs is left for future work"; our Safra proof is also a model. Real systems: no agent has verified liveness of a real unverified distributed system. -->

---

# Proofs no longer main bottleneck

- Bottleneck: spec + link to impl
- → verifying real systems worth trying

<!-- Answer to "can agents do everything?": proof-writing is solved enough to stop being the bottleneck; that is why verifying real systems is now worth attempting. -->

---

# Gap: distributed liveness


- Spec not free
    - need assume what's fair
- Proofs for model, not impl

---

# Feasibility signal

- Agent re-proved Safra's algorithm
    - token ring for completion detection
    - own invariants + countdown
- Toy; human proof already exists
- gpt-6-astra

<!-- TLAPS. Assumes the token step is not starved forever. -->

---

# Possible targets

- Verified systems as the product
- Agents as the method
- Either:
    - verify a class of systems
    - find liveness bugs

---
layout: center
class: text-center
---

# Appendix

---

# Verus example

```rust
fn transfer(from_balance: &mut u64, to_balance: &mut u64, amount: u64)
    requires
        *old(from_balance) >= amount,
        *old(to_balance) + amount <= u64::MAX,
    ensures
        *from_balance == *old(from_balance) - amount,
        *to_balance == *old(to_balance) + amount,
{
    *from_balance = *from_balance - amount;
    *to_balance = *to_balance + amount;
}
```

<div class="small">Verus: "2 verified, 0 errors"</div>

---

# Weak spec

```rust
fn transfer(from_balance: &mut u64, to_balance: &mut u64, amount: u64)
    requires
        *old(from_balance) >= amount,
    ensures
        *from_balance == *old(from_balance) - amount,
{
    *from_balance = *from_balance - amount;
    // bug: amount never reaches to_balance
}
```

<div class="small">Verus: "2 verified, 0 errors"; money vanishes</div>

---

# Sources

<div class="small">

- [Anthropic + Theorem, Oct 22](https://www.anthropic.com/research/formal-verification), Claude (version not stated): "239 entry points across 123 different OSS-Fuzz libraries"; "Our proofs are about single-threaded, static RISC-V binaries linked against musl libc."
- [TLA-Verus](https://arxiv.org/abs/2610.14923), best models Opus-5, GPT-5.6-sol-pro: "Connecting implementations to the specifications via refinement proofs is left for future work."
- IronFleet, SOSP 2015: about 3.7 person-years for the method plus two systems
- Safra proof: gpt-6-astra; TLAPS checked all 434 steps; a human proof exists in tlaplus/Examples
- Summer: proofs Claude Sonnet 4.6; specs Codex CLI with gpt-5.5

</div>
