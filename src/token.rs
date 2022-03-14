use logos::Logos;

// tokens - basically lil data structures the entire program is split up into
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // boolean value (true/false)
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),

    // floating point number (with decimal point)
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    // integer number (no decimal point)
    #[regex("[0-9]+", |lex| lex.slice().parse(), priority = 2)]
    Int(i64),

    // string
    #[regex("\"(\\.|[^\"])*\"", |lex| lex.slice().parse())] // we don't need to bother with de-escaping string contents n shit, it'll just be spat back out anyway
    String(String),

    // math operations (also used for comparisons bc im lazy)
    #[token("add", |lex| lex.slice().parse())]
    #[token("sub", |lex| lex.slice().parse())]
    #[token("mul", |lex| lex.slice().parse())]
    #[token("div", |lex| lex.slice().parse())]
    #[token("idiv", |lex| lex.slice().parse())]
    #[token("mod", |lex| lex.slice().parse())]
    #[token("pow", |lex| lex.slice().parse())]
    #[token("equal", |lex| lex.slice().parse())]
    #[token("strictEqual", |lex| lex.slice().parse())]
    #[token("not", |lex| lex.slice().parse())]
    #[token("land", |lex| lex.slice().parse())]
    #[token("lessThanEq", |lex| lex.slice().parse())]
    #[token("lessThan", |lex| lex.slice().parse())]
    #[token("greaterThanEq", |lex| lex.slice().parse())]
    #[token("greaterThan", |lex| lex.slice().parse())]
    #[token("shl", |lex| lex.slice().parse())]
    #[token("shr", |lex| lex.slice().parse())]
    #[token("xor", |lex| lex.slice().parse())]
    #[token("and", |lex| lex.slice().parse())]
    #[token("or", |lex| lex.slice().parse())]
    #[token("flip", |lex| lex.slice().parse())]
    #[token("max", |lex| lex.slice().parse())]
    #[token("min", |lex| lex.slice().parse())]
    #[token("angle", |lex| lex.slice().parse())]
    #[token("len", |lex| lex.slice().parse())]
    #[token("noise", |lex| lex.slice().parse())]
    #[token("abs", |lex| lex.slice().parse())]
    #[token("log10", |lex| lex.slice().parse())]
    #[token("log", |lex| lex.slice().parse())]
    #[token("sin", |lex| lex.slice().parse())]
    #[token("cos", |lex| lex.slice().parse())]
    #[token("tan", |lex| lex.slice().parse())]
    #[token("floor", |lex| lex.slice().parse())]
    #[token("ceil", |lex| lex.slice().parse())]
    #[token("sqrt", |lex| lex.slice().parse())]
    #[token("rand", |lex| lex.slice().parse())]
    Op(String),

    // all the instructions
    #[token("write")]
    InstWrite,

    #[token("read")]
    InstRead,

    #[token("draw")]
    InstDraw,

    #[token("clear")]
    SubInstClear,

    #[token("color")]
    SubInstColor,
    
    #[token("stroke")]
    SubInstStroke,
    
    #[token("line")]
    SubInstLine,
    
    #[token("rect")]
    SubInstRect,
    
    #[token("lineRect")]
    SubInstLineRect,
    
    #[token("poly")]
    SubInstPoly,
    
    #[token("linePoly")]
    SubInstLinePoly,
    
    #[token("triangle")]
    SubInstTriangle,
    
    #[token("image")]
    SubInstImage,

    #[token("drawflush")]
    InstDrawFlush,

    #[token("print")]
    InstPrint,

    #[token("printflush")]
    InstPrintFlush,

    #[token("getlink")]
    InstGetLink,

    #[token("control")]
    InstControl,

    #[token("enabled")]
    SubInstEnabled,
    
    #[token("shoot")]
    SubInstShoot,
    
    #[token("shootp")]
    SubInstShootP,
    
    #[token("configure")]
    SubInstConfigure,

    #[token("radar")]
    InstRadar,

    #[token("sensor")]
    InstSensor,

    #[token("set")]
    InstSet,

    #[token("op")]
    InstOp,

    #[token("end")]
    InstEnd,

    #[token("jump")]
    InstJump,

    #[token("ubind")]
    InstUnitBind,

    #[token("ucontrol")]
    InstUnitControl,

    #[token("idle")]
    SubInstIdle,

    #[token("stop")]
    SubInstStop,

    #[token("move")]
    SubInstMove,

    #[token("approach")]
    SubInstApproach,

    #[token("boost")]
    SubInstBoost,

    #[token("pathfind")]
    SubInstPathFind,

    #[token("target")]
    SubInstTarget,

    #[token("targetp")]
    SubInstTargetP,

    #[token("itemDrop")]
    SubInstItemDrop,

    #[token("itemTake")]
    SubInstItemTake,

    #[token("payDrop")]
    SubInstPayDrop,

    #[token("PayTake")]
    SubInstPayTake,

    #[token("Mine")]
    SubInstMine,

    #[token("flag")]
    SubInstFlag,

    #[token("build")]
    SubInstBuild,

    #[token("getBlock")]
    SubInstGetBlock,

    #[token("within")]
    SubInstWithin,

    #[token("uradar")]
    InstUnitRadar,

    #[token("ulocate")]
    InstUnitLocate,

    #[token("ore")]
    SubInstOre,

    #[token("building")]
    SubInstBuilding,

    #[token("spawn")]
    SubInstSpawn,

    #[token("noop")]
    InstNoOp,

    #[regex("goto")]
    InstGoto,

    #[regex("gosub")]
    InstGosub,

    #[regex("return")]
    InstReturn,

    #[regex("[a-zA-Z0-9]+", |lex| lex.slice().parse())]
    Name(String),

    #[token("always")]
    Always,

    #[regex("\n")]
    Newline,

    #[regex("[[:word:]]+:", |lex| {
        let slice = lex.slice();
        slice[..slice.len() - 1].parse()
    })]
    Label(String),

    #[regex("routine [[:word:]]+:", |lex| {
        let slice = lex.slice();
        slice[8..slice.len() - 1].parse()
    })]
    Subroutine(String),

    #[regex("@[a-zA-Z]+", |lex| lex.slice().parse())]
    SysVar(String),

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r"#.*", logos::skip)]
    Error,
}
