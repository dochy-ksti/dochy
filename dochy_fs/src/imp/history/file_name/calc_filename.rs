pub fn calc_filename(tag : Option<&str>, control : u32, order: &[u32]) -> String{
    let mut s = String::new();

    if let Some(tag) = tag{
        s.push('#');
        s.push_str(&tag);
        s.push('#');
    }
    s.push('_');
    s.push_str(&control.to_string());
    for phase in order {
        s.push('_');
        s.push_str(&*phase.to_string())
    }
    s.push_str(".his");
    s
}