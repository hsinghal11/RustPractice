pub fn mutable_reference(s: &mut String) -> usize {
    s.push_str("worlf");
    s.len()
}
