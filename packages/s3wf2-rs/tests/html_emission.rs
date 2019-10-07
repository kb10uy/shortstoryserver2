use s3wf2::{
    emitter::{html::HtmlEmitter, Emit},
    parser::Parser,
};

#[test]
fn it_escapes_reserved_characters() {
    let parser = Parser::new();
    let mut emitter = HtmlEmitter::new(4);
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);

    let document = parser.parse("<>&\"").unwrap();
    emitter.emit(&mut buffer, &document).unwrap();
    let html = String::from_utf8(buffer).unwrap();

    assert!(html.find("&lt;&gt;&amp;&quot;").is_some());
}
