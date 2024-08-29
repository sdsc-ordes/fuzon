fn main() {
    let file = b"<http://example.com/subject> <http://www.w3.org/2000/01/rdf-schema#label> \"Hello World\" .";
    
    let all_codes = fuzon::query(vec![file.as_ref()]);
    let _hits = fuzon::filter_terms("Hello".to_string(), all_codes);
}
