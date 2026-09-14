//! Check a real card through the portable VM and native dialect translator.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: beauty_check CARD DATA KIT_DIR".into());
    }
    let card = std::fs::read_to_string(&args[1])?;
    let data = serde_json::from_str(&std::fs::read_to_string(&args[2])?)?;
    let mut measured = args[1].ends_with(".octoscript");
    let mut tree = if measured { octoscript_makepad::design::prepare(&card)? }
        else {
            let prepared = octoscript_makepad::l0::prepare(&card, &data, std::path::Path::new(&args[3]))?;
            measured = prepared.native_components;
            prepared.tree
        };
    let elements = octoscript_makepad::l0::inspectable(&mut tree);
    let ui = if measured { octoscript_makepad::design::to_makepad_ui(&tree)? }
        else { octoscript_makepad::to_makepad_l0_ui(&tree) };
    println!(
        "{}",
        serde_json::json!({"ok": true, "nodes": tree.count(),
        "texts": octoscript_makepad::l0::texts(&tree),
        "images": octoscript_makepad::l0::images(&tree), "elements": elements, "dialect": ui})
    );
    Ok(())
}
