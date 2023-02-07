


pub fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}

#[test]
fn test_interpret() {
    assert_eq!(interpret("G()()()()(al)".to_string()), "Gooooal".to_string());
    assert_eq!(interpret("(al)G(al)()()G".to_string()), "alGalooG".to_string());

}