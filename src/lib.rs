pub fn hr(symbols: Vec<String>) {
    let symbols: Vec<String> = if symbols.is_empty() {
        vec![String::from("#")]
    } else {
       symbols
    };

    if let Some((w, _)) = term_size::dimensions() {
        for sym in symbols {
            let size: usize = sym.chars().map(|c| c.len_utf16()).sum();
            let width = w / size;
            let row = std::iter::repeat(sym).take(width).collect::<String>();
            println!("{}", row);
        }
    }
}
