mod sexpr;
mod qexpr;
pub use self::sexpr::*;
pub use self::qexpr::*;
use std::fmt;
use std::collections::HashMap;
use interpreter::eval_sexpr;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    own_map: HashMap<String, Expr>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn empty() -> Env {
        Env {
            own_map: HashMap::new(),
            parent: None,
        }
    }
    // pub fn new() -> Env {
    //     Env {
    //         own_map: HashMap::new(),
    //         parent: parent,
    //     }
    // }

    pub fn get(&self, key: &str) -> Option<Expr> {
        self.own_map
            .get(key)
            .cloned()
            .or_else(|| {
                self.parent
                    .and_then(|p| p.borrow().get(key))
            })
    }

    pub fn define_local<K: Into<String>>(&mut self, key: K, value: Expr) {
        self.own_map.insert(key.into(), value);
    }

    pub fn define_global<K: Into<String>>(&mut self, key: K, value: Expr) {
        let mut e = Rc::new(RefCell::new(self.parent.clone()));
        while let Some(e_inner) = e.borrow().and_then(|e| e.borrow().parent) {
            e = e_inner;
        }
        // `e` is now the top-level environment, i.e., the global env.
        e.borrow_mut().define_local(key, value);
    }
}

pub type InnerFunc = fn(&mut Env, &[Expr]) -> Result<Expr, String>;

pub enum Function {
    Builtin(InnerFunc),
    Lambda(Lambda),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    local_env: Env,
    parameters: Vec<String>,
    body: QExpr,
}

impl Lambda {
    pub fn new(parameters: QExpr, body: QExpr) -> Result<Lambda, String> {
        let mut symbol_parameters = Vec::new();

        for param in parameters.exprs {
            if let Expr::Symbol(s) = param {
                symbol_parameters.push(s);
            } else {
                return Err(format!("expected symbol in parameter list, found {:?}", param));
            }
        }

        Ok(Lambda {
            local_env: Env::new(),
            parameters: symbol_parameters,
            body: body,
        })
    }

    pub fn call(&mut self, env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
        for (i, arg) in arguments.iter().enumerate() {
            // Populate our local environments with the arguments, which are named by the
            // corresponding parameter name.
            self.local_env.insert(self.parameters[i].clone(), arg.clone());
        }

        eval_sexpr(&mut self.local_env, &mut self.body)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(\\ {:?} {:?})", self.parameters, self.body)
    }
}

impl Function {
    pub fn from_builtin(f: InnerFunc) -> Function {
        Function::Builtin(f)
    }

    pub fn call(&mut self, env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
        match *self {
            Function::Builtin(f) => f(env, arguments),
            Function::Lambda(ref mut f) => f.call(env, arguments),
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        match *self {
            Function::Builtin(f) => Function::Builtin(f),
            Function::Lambda(ref lambda) => Function::Lambda(lambda.clone()),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        // XXX: make sure this is correct.
        match *self {
            Function::Builtin(ref builtin) => {
                if let Function::Builtin(ref other_builtin) = *other {
                    *builtin as *const () == *other_builtin as *const ()
                } else {
                    false
                }
            }
            Function::Lambda(ref lambda) => {
                if let Function::Lambda(ref other_lambda) = *other {
                    lambda == other_lambda
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Function::Builtin(f_ptr) => write!(f, "<a Rulis function @ {:p}>", f_ptr as *const ()),
            Function::Lambda(ref lambda) => write!(f, "lambda: {}", lambda),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    SExpr(SExpr),
    QExpr(QExpr),
    Function(Function),
}

impl Expr {
    pub fn as_i64(&self) -> Option<i64> {
        if let Expr::Integer(i) = *self {
            Some(i)
        } else {
            None
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;

        match *self {
            Integer(i) => write!(f, "{}", i),
            Symbol(ref s) => write!(f, "{}", s),
            SExpr(ref sexpr) => write!(f, "{}", sexpr),
            QExpr(ref qexpr) => write!(f, "{}", qexpr),
            Function(ref func) => write!(f, "{}", func),
        }
    }
}
