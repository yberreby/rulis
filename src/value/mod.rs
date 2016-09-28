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

    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Env {
        Env {
            own_map: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, key: &str) -> Option<Expr> {
        self.own_map
            .get(key)
            .cloned()
            .or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|p| p.borrow().get(key))
            })
    }

    pub fn define_local<K: Into<String>>(&mut self, key: K, value: Expr) {
        self.own_map.insert(key.into(), value);
    }

    // BEWARE: there be unsafe code!
    // XXX: needs _thorough_ testing
    pub fn define_global<K: Into<String>>(&mut self, key: K, value: Expr) {
        unsafe {
            // I couldn't find a way to way this work in safe code that wasn't either stupidly
            // inefficient or plainly incorrect.
            let mut e: *mut Env = self as *mut Env;

            while let Some(ref mut p) = (*e).parent {
                e = p.borrow_mut()
                    .parent
                    .as_ref()
                    .map(|e| &mut (*e.borrow_mut()) as *mut Env)
                    .unwrap_or(::std::ptr::null_mut());
            }
            // `e` is now the top-level environment, i.e., the global env.
            (*e).define_local(key, value);
        }
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
    pub fn new(parameters: QExpr, body: QExpr, parent: Rc<RefCell<Env>>) -> Result<Lambda, String> {
        let mut symbol_parameters = Vec::new();

        for param in parameters.exprs {
            if let Expr::Symbol(s) = param {
                symbol_parameters.push(s);
            } else {
                return Err(format!("expected symbol in parameter list, found {:?}", param));
            }
        }

        Ok(Lambda {
            local_env: Env::with_parent(parent),
            parameters: symbol_parameters,
            body: body,
        })
    }

    pub fn call(&mut self, arguments: &[Expr]) -> Result<Expr, String> {
        for (i, arg) in arguments.iter().enumerate() {
            // Populate our local environment with the arguments, which are named by the
            // corresponding parameter name.
            self.local_env.define_local(self.parameters[i].clone(), arg.clone());
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
            // Lambdas should only have access to the environment in which they were defined.
            //
            // The story is different for builtins. We control what they try to access, and
            // guarantee they will only ever access other builtins. Therefore it is (or, at least,
            // should be) fine to pass an env parameter local to the _call site_ to them, because
            // they will not access any local variable, and this env should have a parent reference
            // to the global scope.
            Function::Builtin(f) => f(env, arguments),
            Function::Lambda(ref mut f) => f.call(arguments),
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
