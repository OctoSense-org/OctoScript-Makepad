//! Evaluate Octoscript source in the makepad-script VM and walk the result into a
//! [`UiNode`] tree. This module owns the **only** makepad-script dependency in
//! the render path — backends never touch the VM, they consume `UiNode`.

use makepad_script::apply::*;
use makepad_script::array::ScriptArrayStorage;
use makepad_script::makepad_live_id::*;
use makepad_script::traits::*;
use makepad_script::*;
use octoscript_node::{Attrs, NodeKind, UiNode};

/// Evaluate a complete document under an instruction budget, rejecting it if
/// evaluation raised any error — even one a later expression recovered from.
/// Native calls and parsing are outside the budget. This is `ScriptVm::eval_checked`
/// from the makepad lineage the L0 work was written against, expressed with the
/// APIs the workspace's pinned makepad revision has (`with_instruction_limit` and
/// the captured-error sink), so both stay usable from one call site.
pub fn eval_checked(vm: &mut ScriptVm, script_mod: ScriptMod, limit: usize) -> Option<ScriptValue> {
    let previous_sink = vm.bx.captured_errors.replace(Vec::new());
    let value = vm.with_instruction_limit(limit, |vm| vm.eval(script_mod));
    let errors = vm.take_errors();
    vm.bx.captured_errors = previous_sink;
    (!value.is_nil() && !value.is_err() && errors.is_empty()).then_some(value)
}

/// Evaluate Octoscript `src` and walk it into a `UiNode` tree.
///
/// `register` runs against the fresh VM *before* evaluation so the host can
/// inject its capabilities (e.g. a `fetch` function) as globals. Pass a no-op
/// (`|_| {}`) if the script needs none. Pure L0 helpers are installed afterward.
/// Returns `None` for evaluation errors, exhausted execution/tree budgets or any
/// malformed node. A failed child invalidates the entire tree.
pub fn build(src: &str, register: impl FnOnce(&mut ScriptVm)) -> Option<UiNode> {
    // No host state and no std slot: the renderer only needs the VM itself.
    let mut host = ScriptVmHost::new((), ());
    let vm = &mut ScriptVm {
        host: &mut host,
        bx: Box::new(ScriptVmBase::new()),
    };

    register(vm);
    let modules = vm.bx.heap.modules;
    let injected_sys = vm
        .bx
        .injected_globals
        .get(&id!(sys))
        .and_then(|v| v.as_object());
    let sys = injected_sys
        .or_else(|| {
            vm.bx
                .heap
                .value(modules, id!(sys).into(), NoTrap)
                .as_object()
        })
        .unwrap_or_else(|| vm.new_module(id!(sys)));
    crate::l0_helpers::install(vm, sys);
    vm.set_injected_global(id!(sys), sys.into());

    let value = eval_checked(
        vm,
        ScriptMod {
            cargo_manifest_path: String::new(),
            module_path: String::from("octoscript"),
            file: String::from("octoscript.octoscript"),
            line: 0,
            column: 0,
            code: src.to_string(),
            values: Vec::new(),
        },
        octoscript_node::MAX_EVAL_INSTRUCTIONS,
    )?;

    if value.is_nil() {
        return None;
    }
    walk(vm, value, 0)
}

/// One DSL object → one `UiNode`, recursing into `c`.
fn walk(vm: &mut ScriptVm, value: ScriptValue, depth: usize) -> Option<UiNode> {
    let mut remaining = octoscript_node::MAX_TREE_NODES;
    walk_inner(vm, value, depth, &mut remaining)
}

fn walk_inner(
    vm: &mut ScriptVm,
    value: ScriptValue,
    depth: usize,
    remaining: &mut usize,
) -> Option<UiNode> {
    if depth > octoscript_node::MAX_TREE_DEPTH || *remaining == 0 {
        return None;
    }
    *remaining -= 1;
    let tag = string_prop(vm, value, id!(t)).unwrap_or_default();
    let kind = NodeKind::from_tag(&tag)?;

    let attrs = Attrs {
        // `text_prop`, not `string_prop`: a card may bind a declared NUMBER here.
        //
        // `string_prop` returns None for a number, so a node whose text was one
        // arrived with no text at all and rendered as an empty row — present,
        // laid out, and blank. Measured with a card reading `sys.gps`, which
        // answers numbers: four labelled rows, four empty values, and nothing to
        // say whether the device had a fix or the call had failed.
        //
        // The same fix was already made for `variant` after the kit's `variant: 2`
        // arrived as nothing and every weather icon fell back to a default sun.
        // This is the same defect one slot over.
        text: text_prop(vm, value, id!(text)),
        label: string_prop(vm, value, id!(label)),
        placeholder: string_prop(vm, value, id!(placeholder)),
        focused: int_prop(vm, value, id!(focused)),
        password: int_prop(vm, value, id!(password)),
        id: string_prop(vm, value, id!(id)),
        tapto: string_prop(vm, value, id!(tapto)),
        src: string_prop(vm, value, id!(src)),
        fit: int_prop(vm, value, id!(fit)),
        w: f32_prop(vm, value, id!(w)),
        h: f32_prop(vm, value, id!(h)),
        fitw: int_prop(vm, value, id!(fitw)),
        fith: int_prop(vm, value, id!(fith)),
        fillw: int_prop(vm, value, id!(fillw)),
        fillh: int_prop(vm, value, id!(fillh)),
        size: f32_prop(vm, value, id!(size)),
        weight: int_prop(vm, value, id!(weight)),
        icon: int_prop(vm, value, id!(icon)),
        icon_name: string_prop(vm, value, id!(icon)),
        color: u32_prop(vm, value, id!(color)),
        bg: u32_prop(vm, value, id!(bg)),
        bg2: u32_prop(vm, value, id!(bg2)),
        texture: string_prop(vm, value, id!(texture)),
        texture_alpha: f32_prop(vm, value, id!(texture_alpha)),
        texture_scale: f32_prop(vm, value, id!(texture_scale)),
        shadowcolor: u32_prop(vm, value, id!(shadowcolor)),
        shadowblur: f32_prop(vm, value, id!(shadowblur)),
        shadowdx: f32_prop(vm, value, id!(shadowdx)),
        shadowdy: f32_prop(vm, value, id!(shadowdy)),
        family: string_prop(vm, value, id!(family)),
        font_src: string_prop(vm, value, id!(font_src)),
        font_asc: f32_prop(vm, value, id!(font_asc)),
        blur: f32_prop(vm, value, id!(blur)),
        font_desc: f32_prop(vm, value, id!(font_desc)),
        image_width: f32_prop(vm, value, id!(image_width)),
        image_height: f32_prop(vm, value, id!(image_height)),
        line_height: f32_prop(vm, value, id!(line_height)),
        tracking: f32_prop(vm, value, id!(tracking)),
        gradient_across: int_prop(vm, value, id!(gradient_across)),
        radius: f32_prop(vm, value, id!(radius)),
        elevation: f32_prop(vm, value, id!(elevation)),
        pad: f32_prop(vm, value, id!(pad)),
        padx: f32_prop(vm, value, id!(padx)),
        pady: f32_prop(vm, value, id!(pady)),
        padleft: f32_prop(vm, value, id!(padleft)),
        padtop: f32_prop(vm, value, id!(padtop)),
        padbottom: f32_prop(vm, value, id!(padbottom)),
        spacing: f32_prop(vm, value, id!(spacing)),
        margin: f32_prop(vm, value, id!(margin)),
        marginx: f32_prop(vm, value, id!(marginx)),
        marginy: f32_prop(vm, value, id!(marginy)),
        margintop: f32_prop(vm, value, id!(margintop)),
        marginbottom: f32_prop(vm, value, id!(marginbottom)),
        border: f32_prop(vm, value, id!(border)),
        bordercolor: u32_prop(vm, value, id!(bordercolor)),
        variant: text_prop(vm, value, id!(variant)),
        kit: string_prop(vm, value, id!(kit)),
        kit_index: int_prop(vm, value, id!(kit_index)),
        enabled: int_prop(vm, value, id!(enabled)),
        key: string_prop(vm, value, id!(key)),
        action: string_prop(vm, value, id!(action)),
        items: string_prop(vm, value, id!(items)),
        selected: int_prop(vm, value, id!(selected)),
        hint: string_prop(vm, value, id!(hint)),
        title: string_prop(vm, value, id!(title)),
        lines: int_prop(vm, value, id!(lines)),
        badge: string_prop(vm, value, id!(badge)),
        count: int_prop(vm, value, id!(count)),
        supporting: string_prop(vm, value, id!(supporting)),
        helper: string_prop(vm, value, id!(helper)),
        group: string_prop(vm, value, id!(group)),
        indeterminate: int_prop(vm, value, id!(indeterminate)),
        min: f32_prop(vm, value, id!(min)),
        max: f32_prop(vm, value, id!(max)),
        step: f32_prop(vm, value, id!(step)),
        value2: f32_prop(vm, value, id!(value2)),
        error: string_prop(vm, value, id!(error)),
        accent: u32_prop(vm, value, id!(accent)),
        markcolor: u32_prop(vm, value, id!(markcolor)),
        value: f32_prop(vm, value, id!(value)),
        total: f32_prop(vm, value, id!(total)),
        align: int_prop(vm, value, id!(align)),
        alignx: f32_prop(vm, value, id!(alignx)),
        aligny: f32_prop(vm, value, id!(aligny)),
        ink: u32_prop(vm, value, id!(ink)),
        on: int_prop(vm, value, id!(on)),
        tap: int_prop(vm, value, id!(tap)),
        lat: num_prop(vm, value, id!(lat)),
        lon: num_prop(vm, value, id!(lon)),
        zoom: num_prop(vm, value, id!(zoom)),
        changeto: string_prop(vm, value, id!(changeto)),
        polyline: string_prop(vm, value, id!(polyline)),
        markers: string_prop(vm, value, id!(markers)),
        route_badge: string_prop(vm, value, id!(route_badge)),
        tilt: num_prop(vm, value, id!(tilt)),
        rotation: num_prop(vm, value, id!(rotation)),
        x: num_prop(vm, value, id!(x)),
        y: num_prop(vm, value, id!(y)),
        // The data-visualisation parameters. The compiler enforces this mirror:
        // `Attrs` is exhaustive here, so a field added to the shared model
        // cannot be silently ignored by this evaluator.
        lo: f32_prop(vm, value, id!(lo)),
        hi: f32_prop(vm, value, id!(hi)),
        rise: f32_prop(vm, value, id!(rise)),
        set: f32_prop(vm, value, id!(set)),
        now: f32_prop(vm, value, id!(now)),
        phase: f32_prop(vm, value, id!(phase)),
        illum: f32_prop(vm, value, id!(illum)),
        span: f32_prop(vm, value, id!(span)),
        symbol: string_prop(vm, value, id!(symbol)),
        range: string_prop(vm, value, id!(range)),
        countries: string_prop(vm, value, id!(countries)),
        indicator: string_prop(vm, value, id!(indicator)),
        years: f32_prop(vm, value, id!(years)),
    };

    let mut children = Vec::new();
    for kid in children_of(vm, value, *remaining)? {
        children.push(walk_inner(vm, kid, depth + 1, remaining)?);
    }

    Some(UiNode {
        kind,
        attrs,
        children,
    })
}

pub fn prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<ScriptValue> {
    vm.bx.heap.value_for_apply(obj, key.into(), &Apply::Eval)
}

pub fn string_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<String> {
    let v = prop(vm, obj, key)?;
    vm.string_with(v, |_vm, s| s.to_string())
}

/// A property that may be text OR a number, read as text.
///
/// `variant` is the one slot on the shared model that carries a CODE rather than
/// a word — a WeatherIcon's condition is the forecast's WMO number — and
/// `string_prop` returns `None` for a number. So the kit set `variant: 2`, the
/// node received nothing, and every weather icon on the card fell back to its
/// default: seven identical suns over a week that was cloudy, rainy and clear.
///
/// Integral values print without a fraction, because this becomes a shader
/// uniform and `2` reads as a code where `2.0` reads as a measurement.
fn text_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<String> {
    if let Some(s) = string_prop(vm, obj, key) {
        return Some(s);
    }
    let v = prop(vm, obj, key)?;
    let n = v.as_number()?;
    Some(if n.is_finite() {
        n.to_string()
    } else {
        "—".into()
    })
}

/// A numeric property, via the VM's own coercion rather than by reading the
/// NaN-boxed representation, so ints, floats and colour literals all work.
pub fn num_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<f64> {
    let v = prop(vm, obj, key)?;
    if v.is_nil() {
        return None;
    }
    let mut out: f64 = f64::NAN;
    <f64 as ScriptApply>::script_apply(&mut out, vm, &Apply::Eval, &mut Scope::default(), v);
    out.is_finite().then_some(out)
}

fn f32_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<f32> {
    num_prop(vm, obj, key)
        .map(|v| v as f32)
        .filter(|v| v.is_finite())
}
fn int_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<i32> {
    num_prop(vm, obj, key).map(|v| v as i32)
}
fn u32_prop(vm: &mut ScriptVm, obj: ScriptValue, key: LiveId) -> Option<u32> {
    num_prop(vm, obj, key).map(|v| v as u32)
}

/// The `c` array's members, copied out so the walk can re-borrow the vm.
///
/// `c` is a ScriptArray, NOT an object holding a vec — arrays are their own heap
/// type in this VM, and treating one as an object drops the entire subtree
/// silently.
fn children_of(
    vm: &mut ScriptVm,
    value: ScriptValue,
    remaining: usize,
) -> Option<Vec<ScriptValue>> {
    let Some(c) = prop(vm, value, id!(c)).filter(|c| !c.is_nil()) else {
        return Some(Vec::new());
    };
    let arr = c.as_array()?;
    match vm.bx.heap.array_storage(arr) {
        ScriptArrayStorage::ScriptValue(v) if v.len() <= remaining => {
            Some(v.iter().copied().collect())
        }
        _ => None,
    }
}

/// Register a native function on the VM and return it as a value ready to inject
/// as a global (e.g. `vm.set_injected_global(id!(fetch), add_global_fn(vm, …))`).
/// The closure receives the call's argument object as a value; read it with
/// [`string_prop`] / [`num_prop`].
pub fn add_global_fn<F>(vm: &mut ScriptVm, args: &[(LiveId, ScriptValue)], f: F) -> ScriptValue
where
    F: Fn(&mut ScriptVm, ScriptValue) -> ScriptValue + 'static,
{
    let base = &mut *vm.bx;
    let mut native = base.code.native.borrow_mut();
    let obj = native.add_fn(&mut base.heap, args, move |vm, args| f(vm, args.into()));
    obj.into()
}
