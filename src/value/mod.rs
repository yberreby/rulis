mod sexpr;
mod qexpr;
pub use self::sexpr::*;
pub use self::qexpr::*;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use interpreter::eval_sexpr;

#[derive(Clone, Debug, PartialEq)]
pub struct EnvPtr {
    ptr: Rc<RefCell<EnvInner>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvInner {
    own_map: HashMap<String, Expr>,
    parent: Option<EnvPtr>,
}

impl EnvPtr {
    pub fn empty() -> EnvPtr {
        EnvPtr {
            ptr: Rc::new(RefCell::new(EnvInner {
                own_map: HashMap::new(),
                parent: None,
            })),
        }
    }

    pub fn with_parent(parent: EnvPtr) -> EnvPtr {
        EnvPtr {
            ptr: Rc::new(RefCell::new(EnvInner {
                own_map: HashMap::new(),
                parent: Some(parent),
            })),
        }
    }

    pub fn get(&self, key: &str) -> Option<Expr> {
        self.ptr
            .borrow()
            .own_map
            .get(key)
            .cloned()
            .or_else(|| {
                self.ptr
                    .borrow()
                    .parent
                    .as_ref()
                    .and_then(|p| p.get(key))
            })
    }

    /// Return either the topmost environment in `self`'s lineage, or `self` if
    /// `self` had no parents.
    pub fn top_level_env(&self) -> EnvPtr {
        let mut e: EnvPtr = self.clone();

        loop {
            let e_copy = e.clone();
            let parent = e_copy.ptr.borrow().parent.clone();
            if let Some(ref parent_ptr) = parent {
                // Note that we're cloning a ref-counted pointer here. Not an environment.
                e = parent_ptr.clone();
            } else {
                return e;
            }
        }
    }

    pub fn define_local<K: Into<String>>(&self, key: K, value: Expr) {
        self.ptr.borrow_mut().own_map.insert(key.into(), value);
    }

    pub fn define_global<K: Into<String>>(&self, key: K, value: Expr) {
        self.top_level_env().define_local(key, value);
    }
}

pub type InnerFunc = fn(EnvPtr, Vec<Expr>) -> Result<Expr, String>;

pub enum Function {
    Builtin(InnerFunc),
    Lambda(Lambda),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    local_env_ptr: EnvPtr,
    parameters: Vec<String>,
    body: QExpr,
}

impl Lambda {
    pub fn new(parameters: QExpr, body: QExpr, parent: EnvPtr) -> Result<Lambda, String> {
        let mut symbol_parameters = Vec::new();

        for param in parameters.exprs {
            if let Expr::Symbol(s) = param {
                symbol_parameters.push(s);
            } else {
                return Err(format!("expected symbol in parameter list, found {:?}", param));
            }
        }

        Ok(Lambda {
            local_env_ptr: EnvPtr::with_parent(parent),
            parameters: symbol_parameters,
            body: body,
        })
    }

    pub fn call(&mut self, arguments: Vec<Expr>) -> Result<Expr, String> {
        let total_parameter_count = self.parameters.len();

        if arguments.len() > self.parameters.len() {
            return Err(format!("too many arguments passed to lambda: found {}, expected at most \
                                {}",
                               arguments.len(),
                               self.parameters.len()));
        }

        // perf: we might want to store a reversed parameters Vec and remove elements from its end
        // with `.pop()`.
        for arg in arguments.iter() {
            debug!("params: {:?}\nargs: {:?}", self.parameters, arguments);
            let next_param = self.parameters.remove(0);

            // Populate our local environment with the arguments, which are named by the
            // corresponding parameter name.
            self.local_env_ptr.define_local(next_param, arg.clone());
        }

        if arguments.len() == total_parameter_count {
            eval_sexpr(self.local_env_ptr.clone(), self.body.exprs.clone())
        } else {
            assert!(arguments.len() < total_parameter_count,
                    "argument count should not be greater than or equal to parameter count at \
                     this point. Arguments: {:?}; remaining parameters: {:?}; total parameter \
                     count: {}",
                    arguments,
                    self.parameters,
                    total_parameter_count);
            // This clone will have all the partial parameters bound to it.
            Ok(Expr::Function(Function::Lambda(self.clone())))
        }
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(\\ {:?} {:?}); own_map: {:?}",
               self.parameters,
               self.body,
               self.local_env_ptr.ptr.borrow().own_map)
    }
}

impl Function {
    pub fn from_builtin(f: InnerFunc) -> Function {
        Function::Builtin(f)
    }

    pub fn call(&mut self, env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
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
