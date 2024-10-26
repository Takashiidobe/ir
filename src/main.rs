use ir::{
    error::EvalError, expr::Expr, optimizer::Optimizer, printer::Printer, serializer::Serdes,
    stmt::Stmt, vm::VM,
};

fn main() -> Result<(), EvalError> {
    let program = vec![
        // Stmt::Print(Expr::Add(
        //     Box::new(Expr::Add(1.into(), 4.into())),
        //     Box::new(Expr::Add(2.into(), 5.into())),
        // )),
        // Stmt::Expr(Expr::Not(false.into())),
        // Stmt::Print(Expr::Not(Box::new(Expr::Not(false.into())))),
        // Stmt::Print(Expr::Sub(5.into(), 1.into())),
        // Stmt::Print(Expr::Mul(5.into(), 2.into())),
        // Stmt::Print(Expr::Div(10.into(), 2.into())),
        // Stmt::Print(Expr::Add("Hi ".into(), "World".into())),
        // Stmt::Print(Expr::And("".into(), "World".into())),
        // Stmt::Print(Expr::Or("".into(), "World".into())),
        // Stmt::Print(Expr::Literal(Value::Array(vec![
        //     Expr::Add(1.into(), 2.into()),
        //     "hi".into(),
        //     vec![1.into(), "world".into()].into(),
        // ]))),
        // Stmt::If(
        //     true.into(),
        //     vec![Stmt::Print(Expr::Div(10.into(), 2.into()))],
        // ),
        // Stmt::If(
        //     false.into(),
        //     vec![Stmt::Print(Expr::Div(10.into(), 5.into()))],
        // ),
        // Stmt::Print(Expr::EqualEqual(
        //     Box::new(Expr::Literal(Value::Num(5.into()))),
        //     Box::new(Expr::Add(3.into(), 2.into())),
        // )),
        // Stmt::Assign("x".to_string(), Expr::Add(3.into(), 2.into())),
        // Stmt::Print(Expr::Var("x".into())),
        // Stmt::Assign(
        //     "x".to_string(),
        //     Expr::Add(Box::new(Expr::Var("x".into())), 2.into()),
        // ),
        // Stmt::Print(Expr::Var("x".into())),
        // Stmt::Block(vec![
        //     Stmt::Print(Expr::Var("x".into())),
        //     Stmt::Assign("x".to_string(), "hi".into()),
        //     Stmt::Print(Expr::Var("x".into())),
        //     Stmt::Block(vec![
        //         Stmt::Print(Expr::Var("x".into())),
        //         Stmt::Assign("x".to_string(), "hello".into()),
        //         Stmt::Print(Expr::Var("x".into())),
        //     ]),
        //     Stmt::Print(Expr::Var("x".into())),
        // ]),
        Stmt::Func(
            "new_fn".to_string(),
            vec!["x".to_string()],
            vec![Stmt::If(
                Expr::LessThanEqual(Expr::Var("x".into()).into(), 3.into()),
                vec![
                    Stmt::Print(Expr::Var("x".into())),
                    Stmt::Expr(Expr::AddAssign(Expr::Var("x".into()).into(), 1.into())),
                    Stmt::Expr(Expr::Call("new_fn".into(), vec![Expr::Var("x".into())])),
                ],
            )],
        ),
        Stmt::Print(Expr::Call("new_fn".to_string(), vec![0.into()])),
    ];
    let printer = Printer::new(&program);
    println!("Original Program:\n{}", printer);
    let mut language = VM::default();
    println!("Original Program evaled");
    language.eval(&program)?;

    let optimized_program = Optimizer::optimize(&program);
    let printer = Printer::new(&optimized_program);
    println!("Optimized Program:\n{}", printer);

    let mut language = VM::default();
    println!("Optimized Program evaled");
    language.eval(&optimized_program)?;

    // let serialized = Serdes::serialize(program);
    // std::fs::write("./a.out", &serialized).unwrap();

    // let serialized = std::fs::read("./a.out").unwrap();

    // let serded = Serdes::deserialize(&serialized);
    // let printer = Printer::new(&serded);
    // println!("Serded program:\n{}", printer);

    Ok(())
}
