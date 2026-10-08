//! Interpreted per-point series vs `d3.series` (the Splash kernel JIT).
//!
//! Run: `cargo run --release -p makepad-d3 --example series_bench`
//!
//! Three measurements per size, for the same wave function:
//! - interp: the status-quo app pattern — a script for-loop calling the fn
//!   per point into a script array (one eval, timed whole);
//! - d3.series end to end: compile the fn as a kernel, run it natively,
//!   hand back a script array (first call pays the compile; the second
//!   call with a changed `t` hits the kernel cache);
//! - the kernel phases alone (compile / native run / array build), timed
//!   from Rust around `vm_kernel` directly.

use std::time::Instant;

use makepad_script::*;
use makepad_script_compute::kernel::MathMode;
use makepad_script_compute::vm_kernel::{self, Decl, Entry, VmKernel};
use makepad_script_compute::Backend;

const FNS: &str = "use mod.math.*\n\
    let dx = 0.001\n\
    fn wave(x) { return sin(x * 3.0) * 0.8 + sin(x * 37.0) * 0.1 + x * 0.05 }\n\
    fn gen(i) { out[i] = wave(float(i) * dx) + t * 0.0 }\n";

fn vm_new() -> ScriptVm<'static> {
    let host = Box::leak(Box::new(ScriptVmHost::new(0i32, ())));
    ScriptVm { host, bx: Box::new(ScriptVmBase::new()) }
}

fn eval(vm: &mut ScriptVm, name: &str, code: String) -> ScriptValue {
    let v = vm.eval(ScriptMod { file: name.into(), code, ..Default::default() });
    let errs = vm.take_errors();
    assert!(errs.is_empty(), "{name}: {errs:?}");
    v
}

fn main() {
    for n in [10_000usize, 100_000, 1_000_000] {
        println!("== n = {n} ==");
        let mut vm = vm_new();
        makepad_d3::octoscript::script_mod(&mut vm);
        makepad_d3::octoscript::series::script_mod(&mut vm);

        // The status-quo app pattern, interpreted.
        let t0 = Instant::now();
        let interp = eval(
            &mut vm,
            "interp",
            format!("{FNS}let t = 0.0\nlet out = []\nfor i in 0..{n} {{ out.push(wave(i * dx)) }}\nout\n"),
        );
        let t_interp = t0.elapsed();
        let t0 = Instant::now();
        let interp_vec = makepad_d3::octoscript::vm_data::to_f64_vec(&mut vm, interp).expect("interp out");
        let t_ingest_interp = t0.elapsed();
        assert_eq!(interp_vec.len(), n);

        // d3.series end to end: first call compiles, second call (new t)
        // hits the kernel cache. Both calls inside one eval would hide the
        // split, so each is its own eval against the same document fns.
        let t0 = Instant::now();
        let s1 = eval(&mut vm, "series1", format!("{FNS}mod.d3.series(gen, {n}, 0.0)\n"));
        let t_series_first = t0.elapsed();
        let t0 = Instant::now();
        let gen = eval(&mut vm, "genref", format!("{FNS}gen\n"));
        let gen_obj = gen.as_object().expect("gen fn");
        let t_redefine = t0.elapsed();

        // Kernel phases, timed around vm_kernel directly on the same fn.
        let vk = VmKernel {
            decls: vec![
                Decl::Output { name: "out".into(), ty: "f32".into(), stride: None, offset: None, buffer: None },
                Decl::Param { name: "t".into(), default: 0.0, range: None },
            ],
            entry: Entry::Element,
            entry_fn: gen_obj,
            math: MathMode::Fast,
            uses: Vec::new(),
            bind: Vec::new(),
        };
        let t0 = Instant::now();
        let (kernel, _src) = vm_kernel::compile(&vm, &vk, &[], Backend::Native, &[]).unwrap_or_else(|e| panic!("{e:?}"));
        let t_compile = t0.elapsed();

        let mut out = vec![0f32; n];
        let t0 = Instant::now();
        let mut call = kernel.call();
        call.set_param("t", 1.0);
        call.output("out", &mut out).unwrap();
        call.run(n).unwrap();
        drop(call);
        let t_native_1t = t0.elapsed();

        let threads = std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1).min(8);
        let t0 = Instant::now();
        let mut call = kernel.call();
        call.set_param("t", 2.0);
        call.output("out", &mut out).unwrap();
        call.run_parallel(n, threads).unwrap();
        drop(call);
        let t_native_mt = t0.elapsed();

        let t0 = Instant::now();
        let array = vm.bx.heap.new_array();
        for v in &out {
            vm.bx.heap.array_push_unchecked(array, ScriptValue::from_f32(*v));
        }
        let t_array = t0.elapsed();

        let t0 = Instant::now();
        let series_vec = makepad_d3::octoscript::vm_data::to_f64_vec(&mut vm, s1).expect("series out");
        let t_ingest_series = t0.elapsed();
        assert_eq!(series_vec.len(), n);

        // The same curve from both paths (interp is f64 libm, the kernel is
        // f32 polynomial math: compare loosely).
        for i in [0usize, n / 3, n - 1] {
            let d = (interp_vec[i] - series_vec[i]).abs();
            assert!(d < 1e-3, "point {i}: interp {} vs kernel {}", interp_vec[i], series_vec[i]);
        }

        let per = |d: std::time::Duration| d.as_secs_f64() * 1e9 / n as f64;
        println!("  interp loop (eval)          {:>10.2?}  ({:>7.1} ns/pt)", t_interp, per(t_interp));
        println!("  d3.series end-to-end, cold  {:>10.2?}  ({:>7.1} ns/pt)", t_series_first, per(t_series_first));
        println!("  kernel compile (once)       {:>10.2?}", t_compile);
        println!("  kernel run, 1 thread        {:>10.2?}  ({:>7.1} ns/pt)", t_native_1t, per(t_native_1t));
        println!("  kernel run, {threads} threads       {:>10.2?}  ({:>7.1} ns/pt)", t_native_mt, per(t_native_mt));
        println!("  array build (script heap)   {:>10.2?}  ({:>7.1} ns/pt)", t_array, per(t_array));
        println!("  ingest to_f64_vec: interp {:?} / series {:?}", t_ingest_interp, t_ingest_series);
        println!("  (fn re-eval for the direct-kernel phase: {:?})", t_redefine);
        println!(
            "  speedup: interp vs (run+array) 1t = {:.0}x, vs end-to-end cold = {:.1}x",
            t_interp.as_secs_f64() / (t_native_1t.as_secs_f64() + t_array.as_secs_f64()),
            t_interp.as_secs_f64() / t_series_first.as_secs_f64(),
        );
    }
}
