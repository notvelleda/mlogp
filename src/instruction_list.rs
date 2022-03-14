use crate::instruction_compiler::*;
use crate::value::Value;
use crate::token::Token;

// sets up a vector with all possible instructions
pub fn create_instructions() -> Vec<Box<dyn InstructionCompiler>> {
    vec![
        // all the different kinds of instructions!
        // write  value: float, cell: Building, address: int
        Box::new(Instruction {
            name: "write".to_string(),
            token: Token::InstWrite,
            arguments: vec![
                Value::Float("value".to_string()),
                Value::Name("cell".to_string(), "Building".to_string()),
                Value::Int("address".to_string())
            ],
            super_instruction_name: None,
        }),
        // read  store: variable, cell: Building, address: int
        Box::new(Instruction {
            name: "read".to_string(),
            token: Token::InstRead,
            arguments: vec![
                Value::Variable("store".to_string()),
                Value::Name("cell".to_string(), "Building".to_string()),
                Value::Int("address".to_string())
            ],
            super_instruction_name: None,
        }),
        // draw (group)
        Box::new(InstructionGroup {
            name: "draw".to_string(),
            token: Token::InstDraw,
            sub_instructions: vec![
                // clear  r: int, g: int, b: int
                Instruction {
                    name: "clear".to_string(),
                    token: Token::SubInstClear,
                    arguments: vec![
                        Value::Int("red".to_string()),
                        Value::Int("green".to_string()),
                        Value::Int("blue".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // color  r: int, g: int, b: int
                Instruction {
                    name: "color".to_string(),
                    token: Token::SubInstColor,
                    arguments: vec![
                        Value::Int("red".to_string()),
                        Value::Int("green".to_string()),
                        Value::Int("blue".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // stroke  width: int
                Instruction {
                    name: "stroke".to_string(),
                    token: Token::SubInstStroke,
                    arguments: vec![
                        Value::Int("width".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // line  x1: int, y1: int, x2: int, y2: int
                Instruction {
                    name: "line".to_string(),
                    token: Token::SubInstLine,
                    arguments: vec![
                        Value::Int("x1".to_string()),
                        Value::Int("y1".to_string()),
                        Value::Int("x2".to_string()),
                        Value::Int("y2".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // rect  x: int, y: int, w: int, h: int
                Instruction {
                    name: "rect".to_string(),
                    token: Token::SubInstRect,
                    arguments: vec![
                        Value::Int("x".to_string()),
                        Value::Int("y".to_string()),
                        Value::Int("w".to_string()),
                        Value::Int("h".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // lineRect  x: int, y: int, w: int, h: int
                Instruction {
                    name: "lineRect".to_string(),
                    token: Token::SubInstLineRect,
                    arguments: vec![
                        Value::Int("x".to_string()),
                        Value::Int("y".to_string()),
                        Value::Int("w".to_string()),
                        Value::Int("h".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // poly  x: int, y: int, sides: int, radius: float, rotation: float
                Instruction {
                    name: "poly".to_string(),
                    token: Token::SubInstPoly,
                    arguments: vec![
                        Value::Int("x".to_string()),
                        Value::Int("y".to_string()),
                        Value::Int("sides".to_string()),
                        Value::Float("radius".to_string()),
                        Value::Float("rotation".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // linePoly  x: int, y: int, sides: int, radius: float, rotation: float
                Instruction {
                    name: "linePoly".to_string(),
                    token: Token::SubInstLinePoly,
                    arguments: vec![
                        Value::Int("x".to_string()),
                        Value::Int("y".to_string()),
                        Value::Int("sides".to_string()),
                        Value::Float("radius".to_string()),
                        Value::Float("rotation".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // triangle  x1: int, y1: int, x2: int, y2: int, x3: int, y3: int
                Instruction {
                    name: "triangle".to_string(),
                    token: Token::SubInstTriangle,
                    arguments: vec![
                        Value::Int("x1".to_string()),
                        Value::Int("y1".to_string()),
                        Value::Int("x2".to_string()),
                        Value::Int("y2".to_string()),
                        Value::Int("x3".to_string()),
                        Value::Int("y3".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                },
                // image  x: int, y: int, image: UnlockableContent, size: float, rotation: float
                Instruction {
                    name: "image".to_string(),
                    token: Token::SubInstImage,
                    arguments: vec![
                        Value::Int("x".to_string()),
                        Value::Int("y".to_string()),
                        Value::Name("image".to_string(), "UnlockableContent".to_string()),
                        Value::Float("size".to_string()),
                        Value::Float("rotation".to_string())
                    ],
                    super_instruction_name: Some("draw".to_string()),
                }
            ]
        }),
        // drawflush  display: Building
        Box::new(Instruction {
            name: "drawflush".to_string(),
            token: Token::InstDrawFlush,
            arguments: vec![
                Value::Name("display".to_string(), "Building".to_string())
            ],
            super_instruction_name: None,
        }),
        // print  text: string
        Box::new(Instruction {
            name: "print".to_string(),
            token: Token::InstPrint,
            arguments: vec![
                Value::String("text".to_string())
            ],
            super_instruction_name: None,
        }),
        // printflush  msgblock: Building
        Box::new(Instruction {
            name: "printflush".to_string(),
            token: Token::InstPrintFlush,
            arguments: vec![
                Value::Name("msgblock".to_string(), "Building".to_string())
            ],
            super_instruction_name: None,
        }),
        // getlink  store: variable, index: int
        Box::new(Instruction {
            name: "getlink".to_string(),
            token: Token::InstGetLink,
            arguments: vec![
                Value::Variable("store".to_string()),
                Value::Int("index".to_string())
            ],
            super_instruction_name: None,
        }),
        // control (group)
        Box::new(InstructionGroup {
            name: "control".to_string(),
            token: Token::InstControl,
            sub_instructions: vec![
                // enabled  target: Building, enabled: bool
                Instruction {
                    name: "enabled".to_string(),
                    token: Token::SubInstEnabled,
                    arguments: vec![
                        Value::Name("target".to_string(), "Building".to_string())
                    ],
                    super_instruction_name: Some("control".to_string()),
                },
                // shoot  turret: Building, x: float, y: float, shoot: bool
                Instruction {
                    name: "shoot".to_string(),
                    token: Token::SubInstShoot,
                    arguments: vec![
                        Value::Name("turret".to_string(), "Building".to_string()),
                        Value::Float("x".to_string()), Value::Float("y".to_string()),
                        Value::Bool("shoot".to_string())
                    ],
                    super_instruction_name: Some("control".to_string()),
                },
                // shootp  turret: Building, target: Healthc, shoot: bool
                Instruction {
                    name: "shootp".to_string(),
                    token: Token::SubInstShootP,
                    arguments: vec![
                        Value::Name("turret".to_string(), "Building".to_string()),
                        Value::Name("target".to_string(), "Healthc".to_string()),
                        Value::Bool("shoot".to_string())
                    ],
                    super_instruction_name: Some("control".to_string()),
                },
                // configure  build: Building, config: Content
                Instruction {
                    name: "configure".to_string(),
                    token: Token::SubInstConfigure,
                    arguments: vec![
                        Value::Name("build".to_string(), "Building".to_string()),
                        Value::Name("config".to_string(), "Content".to_string())
                    ],
                    super_instruction_name: Some("control".to_string()),
                },
                // color  illuminator: Building, r: float, g: float, b: float
                Instruction {
                    name: "color".to_string(),
                    token: Token::SubInstColor,
                    arguments: vec![
                        Value::Name("illuminator".to_string(), "Building".to_string()),
                        Value::Int("red".to_string()),
                        Value::Int("green".to_string()),
                        Value::Int("blue".to_string())
                    ],
                    super_instruction_name: Some("control".to_string()),
                },
            ]
        }),
        // radar  turret: Ranged, prop1: TargetType, prop2: TargetType, prop3: TargetType, order: bool, output: variable
        Box::new(Instruction {
            name: "radar".to_string(),
            token: Token::InstRadar,
            arguments: vec![
                Value::Name("turret".to_string(), "Ranged".to_string()),
                Value::Name("prop1".to_string(), "TargetType".to_string()),
                Value::Name("prop2".to_string(), "TargetType".to_string()),
                Value::Name("prop3".to_string(), "TargetType".to_string()),
                Value::Bool("order".to_string()),
                Value::Variable("output".to_string())
            ],
            super_instruction_name: None,
        }),
        // sensor  store: variable, block: Building, sense: Sensable
        Box::new(Instruction {
            name: "sensor".to_string(),
            token: Token::InstSensor,
            arguments: vec![
                Value::Variable("store".to_string()),
                Value::Name("block".to_string(), "Building".to_string()),
                Value::Name("sense".to_string(), "Sensable".to_string())
            ],
            super_instruction_name: None,
        }),
        // set  varname: variable, value: any
        Box::new(Instruction {
            name: "set".to_string(),
            token: Token::InstSet,
            arguments: vec![
                Value::Variable("varname".to_string()),
                Value::Any("value".to_string())
            ],
            super_instruction_name: None,
        }),
        // end
        Box::new(Instruction {
            name: "end".to_string(),
            token: Token::InstEnd,
            arguments: vec![],
            super_instruction_name: None,
        }),
        // ucontrol (group)
        Box::new(InstructionGroup {
            name: "ucontrol".to_string(),
            token: Token::InstUnitControl,
            sub_instructions: vec![
                // idle
                Instruction {
                    name: "idle".to_string(),
                    token: Token::SubInstIdle,
                    arguments: vec![],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // stop
                Instruction {
                    name: "stop".to_string(),
                    token: Token::SubInstStop,
                    arguments: vec![],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // move  x: float, y: float
                Instruction {
                    name: "move".to_string(),
                    token: Token::SubInstMove,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // approach  x: float, y: float, radius: float
                Instruction {
                    name: "approach".to_string(),
                    token: Token::SubInstApproach,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string()),
                        Value::Float("radius".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // boost  enable: bool
                Instruction {
                    name: "boost".to_string(),
                    token: Token::SubInstBoost,
                    arguments: vec![
                        Value::Bool("enable".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // pathfind
                Instruction {
                    name: "pathfind".to_string(),
                    token: Token::SubInstPathFind,
                    arguments: vec![],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // target  x: float, y: float, shoot: bool
                Instruction {
                    name: "target".to_string(),
                    token: Token::SubInstTarget,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string()),
                        Value::Bool("shoot".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // targetp  target: Healthc, shoot: bool
                Instruction {
                    name: "targetp".to_string(),
                    token: Token::SubInstTargetP,
                    arguments: vec![
                        Value::Name("target".to_string(), "Healthc".to_string()),
                        Value::Bool("shoot".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // itemDrop  to: Building, amount: int
                Instruction {
                    name: "itemDrop".to_string(),
                    token: Token::SubInstItemDrop,
                    arguments: vec![
                        Value::Name("to".to_string(), "Building".to_string()),
                        Value::Int("amount".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // itemTake  from: Building, item: Item, amount: int
                Instruction {
                    name: "itemTake".to_string(),
                    token: Token::SubInstItemTake,
                    arguments: vec![
                        Value::Name("from".to_string(), "Building".to_string()),
                        Value::Name("item".to_string(), "Item".to_string()),
                        Value::Int("amount".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // payDrop
                Instruction {
                    name: "payDrop".to_string(),
                    token: Token::SubInstPayDrop,
                    arguments: vec![],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // payTake  takeUnits: bool
                Instruction {
                    name: "payTake".to_string(),
                    token: Token::SubInstPayTake,
                    arguments: vec![
                        Value::Bool("takeUnits".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // mine  x: f32, y: f32
                Instruction {
                    name: "mine".to_string(),
                    token: Token::SubInstMine,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // flag  flag: float
                Instruction {
                    name: "flag".to_string(),
                    token: Token::SubInstFlag,
                    arguments: vec![
                        Value::Float("flag".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // build  x: float, y: float, block: Block, rotation: int, config: variable
                Instruction {
                    name: "build".to_string(),
                    token: Token::SubInstBuild,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string()),
                        Value::Name("block".to_string(), "Block".to_string()),
                        Value::Int("rotation".to_string()),
                        Value::Variable("config".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // getBlock  x: float, y: float, type: variable (Block), building: variable (Building)
                Instruction {
                    name: "getBlock".to_string(),
                    token: Token::SubInstGetBlock,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string()),
                        Value::Variable("block".to_string()),
                        Value::Variable("building".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
                // within  x: float, y: float, radius: float, result: bool
                Instruction {
                    name: "within".to_string(),
                    token: Token::SubInstWithin,
                    arguments: vec![
                        Value::Float("x".to_string()),
                        Value::Float("y".to_string()),
                        Value::Float("radius".to_string()),
                        Value::Variable("result".to_string())
                    ],
                    super_instruction_name: Some("ucontrol".to_string()),
                },
            ]
        }),
        // uradar  a: Target, b: Target, c: Target, sort: SortType, 0: bool, order: bool, result: variable (Healthc)
        Box::new(Instruction {
            name: "uradar".to_string(),
            token: Token::InstUnitRadar,
            arguments: vec![
                Value::Name("a".to_string(), "Target".to_string()),
                Value::Name("b".to_string(), "Target".to_string()),
                Value::Name("c".to_string(), "Target".to_string()),
                Value::Name("sort".to_string(), "SortType".to_string()),
                Value::Bool("0".to_string()),
                Value::Bool("order".to_string()),
                Value::Variable("result".to_string())
            ],
            super_instruction_name: None,
        }),
        // ulocate (group)
        Box::new(InstructionGroup {
            name: "ulocate".to_string(),
            token: Token::InstUnitLocate,
            sub_instructions: vec![
                // ore  ore: Item, outX: variable (float), outY: variable (float), outFound: variable (bool)
                Instruction {
                    name: "ore".to_string(),
                    token: Token::SubInstOre,
                    arguments: vec![
                        Value::Name("ore".to_string(), "Item".to_string()),
                        Value::Variable("outX".to_string()),
                        Value::Variable("outY".to_string()),
                        Value::Variable("outFound".to_string())
                    ],
                    super_instruction_name: Some("ulocate".to_string()),
                },
                // building  type: BlockFlag, enemy: bool, outX: variable (float), outY: variable (float), outFound: variable (bool)
                Instruction {
                    name: "building".to_string(),
                    token: Token::SubInstBuilding,
                    arguments: vec![
                        Value::Name("type".to_string(), "BlockFlag".to_string()),
                        Value::Bool("enemy".to_string()),
                        Value::Variable("outX".to_string()),
                        Value::Variable("outY".to_string()),
                        Value::Variable("outFound".to_string()),
                        Value::Variable("outBuilding".to_string())
                    ],
                    super_instruction_name: Some("ulocate".to_string()),
                },
                // spawn  outX: variable (float), outY: variable (float), outFound: variable (bool)
                Instruction {
                    name: "spawn".to_string(),
                    token: Token::SubInstSpawn,
                    arguments: vec![
                        Value::Variable("outX".to_string()),
                        Value::Variable("outY".to_string()),
                        Value::Variable("outFound".to_string())
                    ],
                    super_instruction_name: Some("ulocate".to_string()),
                },
            ]
        }),
        // noop
        Box::new(Instruction {
            name: "noop".to_string(),
            token: Token::InstNoOp,
            arguments: vec![],
            super_instruction_name: None,
        }),

        // jump  addr: label, comp: Comp, a: any, <b: any>
        // jump  addr: label, always
        Box::new(InstructionJump {}),

        // goto  addr: label
        Box::new(InstructionGoto {}),

        // gosub  addr: routine
        Box::new(InstructionGosub {}),

        // op  op: op, result: variable, a: any, <b: any>
        Box::new(InstructionOp {}),
    ]
}
