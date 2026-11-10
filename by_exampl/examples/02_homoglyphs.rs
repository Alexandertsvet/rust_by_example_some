//! Омоглифы (homoglyphs) — это символы, которые выглядят одинаково или очень похоже, но имеют разные цифровые коды (Unicode) и значения.
//! 
//! 

pub fn main(){
    homoglyphs();

}
pub fn homoglyphs(){
    println!("=== Омоглифы ===");
    // english
    let x = "X";
    // russian
    let y = "Х";
    println!("x: {}, y: {}", x, y);
    if x == y {
        println!("{} == {} равны", x, y);
    } else {
        println!("{} != {} омоглифы", x, y);
    }

    let homoglyph = String::from("XXXXXXXXХ");
    println!("Строка: {}", homoglyph);
    println!("Размер в символах (chars): {}", homoglyph.chars().count());
    println!("Размер в байтах (len): {}\n", homoglyph.len());

    for (i, c) in homoglyph.chars().enumerate() {
        let mut buf = [0; 4];
        let bytes = c.encode_utf8(&mut buf);
        
        println!(
            "Позиция: {} | Символ: {} | Скрипт: {} | CodePoint: U+{:04X} | UTF-8 байты: {:X?}",
            i + 1,
            c,
            if c.is_ascii() { "Латиница  " } else { "Кириллица " },
            c as u32,
            bytes.as_bytes()
        );
    }

    println!("Строка {}, ASCII: {}.", homoglyph, homoglyph.is_ascii());
}