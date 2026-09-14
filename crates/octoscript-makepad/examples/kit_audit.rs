//! Offline proof that promotion preserves every native property and child.
use std::path::Path;

fn source_tree(
    node: &mut octoscript_render::UiNode,
    selectors: &mut std::collections::BTreeMap<String, bool>,
) -> Result<(), String> {
    if let Some(config) = octoscript_makepad::design::kit_contract(node)? {
        if let Some(items) = config["items"].as_array() {
            for (i, item) in items.iter().enumerate() {
                selectors.insert(
                    item["control"].as_str().unwrap().into(),
                    config["source_selected_index"].as_i64() == Some(i as i64),
                );
            }
            node.attrs.kit_index = None;
        }
    }
    node.attrs.kit = None;
    let mut children = Vec::new();
    for mut child in std::mem::take(&mut node.children) {
        let radio = child.attrs.id.as_deref()
            == node
                .attrs
                .id
                .as_ref()
                .map(|id| format!("{id}_kit_radio"))
                .as_deref();
        let button = child.attrs.id.as_deref()
            == node
                .attrs
                .id
                .as_ref()
                .map(|id| format!("{id}_kit_button"))
                .as_deref();
        if radio || button {
            // Permitted additions are transparent native selection/action
            // controls. Check their complete style and declared initial state.
            let a = &node.attrs;
            let source = format!(
                "{{t:{:?},id:{:?},x:{},y:{},w:{},h:{},{}enabled:1}}",
                if radio { "radio" } else { "button" },
                child.attrs.id.as_ref().unwrap(),
                a.x.unwrap_or(0.),
                a.y.unwrap_or(0.),
                a.w.unwrap_or(0.),
                a.h.unwrap_or(0.),
                if radio {
                    format!(
                        "variant:\"silent\",on:{},",
                        i32::from(
                            *selectors
                                .get(child.attrs.id.as_ref().unwrap())
                                .ok_or("unbound added selector")?
                        )
                    )
                } else {
                    "text:\"\",".into()
                }
            );
            let expected = octoscript_makepad::design::prepare(&source)?;
            if octoscript_makepad::design::to_makepad_ui(&child)?
                != octoscript_makepad::design::to_makepad_ui(&expected)?
            {
                return Err("invalid added native control".into());
            }
        } else {
            source_tree(&mut child, selectors)?;
            children.push(child);
        }
    }
    node.children = children;
    Ok(())
}

fn same_line(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    // The source VM and JSON parser can round a decimal at opposite sides of
    // the last f64 bit. Bound this exception to geometry, below 1e-8 points.
    if !["abs_pos: ", "width: "]
        .iter()
        .any(|p| a.starts_with(p) && b.starts_with(p))
    {
        return false;
    }
    let split = |s: &str| {
        s.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
    };
    match (split(a), split(b)) {
        (Ok(a), Ok(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| (a - b).abs() < 1e-8),
        _ => false,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: kit_audit CARDS SOURCE_DESIGNS KIT_DIR OUTPUT".into());
    }
    let mut paths = std::fs::read_dir(&args[1])?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "card"))
        .collect::<Vec<_>>();
    paths.sort();
    let mut rows = Vec::new();
    for path in paths {
        let name = path.file_stem().unwrap().to_str().unwrap();
        let result = (|| -> Result<usize, String> {
            let card = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let data = serde_json::from_str(
                &std::fs::read_to_string(path.with_extension("data.json"))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            let prepared = octoscript_makepad::l0::prepare(&card, &data, Path::new(&args[3]))?;
            if !prepared.native_components {
                return Err("did not use registered components".into());
            }
            let before =
                std::fs::read_to_string(Path::new(&args[2]).join(format!("{name}.octoscript")))
                    .map_err(|e| e.to_string())?;
            let before = octoscript_makepad::design::prepare(&before)?;
            let expected = octoscript_makepad::design::to_makepad_ui(&before)?;
            let mut preserved = prepared.tree.clone();
            source_tree(&mut preserved, &mut Default::default())?;
            let actual = octoscript_makepad::design::to_makepad_ui(&preserved)?;
            if expected.lines().count() != actual.lines().count()
                || !expected
                    .lines()
                    .zip(actual.lines())
                    .all(|(a, b)| same_line(a, b))
            {
                let mismatch = expected
                    .lines()
                    .zip(actual.lines())
                    .enumerate()
                    .find(|(_, (a, b))| !same_line(a, b));
                return Err(format!(
                    "native widget properties/hierarchy changed during promotion: {mismatch:?}"
                ));
            }
            Ok(prepared.tree.count())
        })();
        let row = match result {
            Ok(nodes) => serde_json::json!({"screen":name,"pass":true,"nodes":nodes}),
            Err(error) => serde_json::json!({"screen":name,"pass":false,"error":error}),
        };
        if row["pass"] != true {
            eprintln!("{row}");
        }
        rows.push(row);
    }
    let pass = !rows.is_empty() && rows.iter().all(|r| r["pass"] == true);
    std::fs::write(
        &args[4],
        serde_json::to_string_pretty(&serde_json::json!({"pass":pass,"screens":rows}))?,
    )?;
    if !pass {
        return Err("kit promotion audit failed".into());
    }
    println!("all component instances preserve the source native widget tree");
    Ok(())
}
