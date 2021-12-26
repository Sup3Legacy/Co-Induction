use z3::{
    ast::{Ast, Int, Bool},
    Config, Context, SatResult, Solver,
};

fn main() {
    let ctx = &Context::new(&Config::default()); // we declare constants in a Context
    let solver = Solver::new(ctx);
    let i = Int::new_const(&ctx, "x");
    let b = Bool::new_const(&ctx, "b");
    //solver.assert(&i._eq(&Int::from_i64(&ctx, 3)));
    solver.assert(&i.le(&Int::from_i64(&ctx, 4)));
    solver.assert(&i.ge(&Int::from_i64(&ctx, 0)));
    //solver.assert(&b._eq(&i._eq(&Int::from_i64(&ctx, 4))));


    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();

    println!("{:?}, {:?}", model.eval(&i, true).unwrap().as_i64().unwrap(), model.eval(&b, true).unwrap());
}
