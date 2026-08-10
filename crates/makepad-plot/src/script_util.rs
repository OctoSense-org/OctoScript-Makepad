// Helpers for parsing Splash ScriptValue arguments in widget script_call methods

use makepad_widgets::*;

/// Read positional argument `index` from a script_call args object.
pub fn script_arg(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<ScriptValue> {
    let args_obj = args.as_object()?;
    let len = vm.bx.heap.vec_len(args_obj);
    if index >= len {
        return None;
    }
    Some(vm.bx.heap.vec_value(args_obj, index, NoTrap))
}

/// Read element `i` of a script sequence (typed array literal or object vec part).
fn seq_value(vm: &mut ScriptVm, value: ScriptValue, i: usize) -> Option<ScriptValue> {
    if let Some(arr) = value.as_array() {
        return Some(vm.bx.heap.array_index_unchecked(arr, i));
    }
    let obj = value.as_object()?;
    Some(vm.bx.heap.vec_value(obj, i, NoTrap))
}

/// Length of a script sequence (typed array literal or object vec part).
fn seq_len(vm: &mut ScriptVm, value: ScriptValue) -> Option<usize> {
    if let Some(arr) = value.as_array() {
        return Some(vm.bx.heap.array_len(arr));
    }
    let obj = value.as_object()?;
    Some(vm.bx.heap.vec_len(obj))
}

/// Parse a script array of numbers into a Vec<f64>.
pub fn script_f64_array(vm: &mut ScriptVm, value: ScriptValue) -> Option<Vec<f64>> {
    let len = seq_len(vm, value)?;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let v = seq_value(vm, value, i)?;
        out.push(v.as_number().unwrap_or(0.0));
    }
    Some(out)
}

/// Parse a script array of arrays of numbers into a Vec<Vec<f64>>.
pub fn script_f64_matrix(vm: &mut ScriptVm, value: ScriptValue) -> Option<Vec<Vec<f64>>> {
    let len = seq_len(vm, value)?;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let row = seq_value(vm, value, i)?;
        out.push(script_f64_array(vm, row)?);
    }
    Some(out)
}

/// Parse a script array of strings into a Vec<String>.
pub fn script_string_array(vm: &mut ScriptVm, value: ScriptValue) -> Option<Vec<String>> {
    let len = seq_len(vm, value)?;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let v = seq_value(vm, value, i)?;
        out.push(script_to_string(vm, v));
    }
    Some(out)
}

/// Convert any script value to its string representation.
pub fn script_to_string(vm: &mut ScriptVm, value: ScriptValue) -> String {
    vm.bx.heap.temp_string_with(|heap, out| {
        heap.cast_to_string(value, out);
        out.to_string()
    })
}

/// Read a positional f64 argument.
pub fn script_arg_f64(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<f64> {
    script_arg(vm, args, index)?.as_number()
}

/// Read a positional string argument.
pub fn script_arg_string(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<String> {
    let v = script_arg(vm, args, index)?;
    Some(script_to_string(vm, v))
}

/// Read a positional bool argument.
pub fn script_arg_bool(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<bool> {
    let v = script_arg(vm, args, index)?;
    if let Some(b) = v.as_bool() {
        return Some(b);
    }
    v.as_number().map(|f| f != 0.0)
}

/// Read a positional array-of-f64 argument.
pub fn script_arg_f64_array(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<Vec<f64>> {
    let v = script_arg(vm, args, index)?;
    script_f64_array(vm, v)
}

/// Read a positional matrix argument.
pub fn script_arg_f64_matrix(vm: &mut ScriptVm, args: &ScriptValue, index: usize) -> Option<Vec<Vec<f64>>> {
    let v = script_arg(vm, args, index)?;
    script_f64_matrix(vm, v)
}

/// Read a positional array-of-string argument.
pub fn script_arg_string_array(
    vm: &mut ScriptVm,
    args: &ScriptValue,
    index: usize,
) -> Option<Vec<String>> {
    let v = script_arg(vm, args, index)?;
    script_string_array(vm, v)
}
