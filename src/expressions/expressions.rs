use chrono::{prelude::*, Duration};

pub enum Expressions {
    Time(Duration),
    Date(DateTime),
    InfixExp {
        left: Box<Expressions>,
        op: String,
        right: Box<Expressions>,
    },
    PostfixExp {
        left: Box<Expressions>,
        op: String,
    },
    PrefixExp {
        op: String,
        right: Box<Expressions>,
    },
    AssignExp {
        name: String,
        right: Box<Expressions>,
    },
    NameExp {
        name: String,
    },
    CondititionalExp {
        condition: Box<Expressions>,
        if_branch: Box<Expressions>,
        else_branch: Box<Expressions>,
    },
    CallExp {
        function: Box<Expressions>,
        args: Vec<Expressions>,
    },
}
