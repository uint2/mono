/// (dwm) static void attach(Client *c);
/// (dwm) static void attachstack(Client *c);
pub fn attach<T>(vec: &mut Vec<T>, value: T) {
    vec.push(value);
}

/// (dwm) static void detach(Client *c);
/// (dwm) static void detachstack(Client *c);
pub fn detach<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    let Some(idx) = vec.iter().position(|v| v == value) else { return };
    vec.remove(idx);
}
