use crate::interpreter::error::RuntimeError;
use crate::interpreter::Result;
use crate::syntax_trees::lox_object::LoxObject;
use crate::token::SmartString;
use std::collections::HashMap;
use std::fmt;
use std::ptr::NonNull;

use std::collections::hash_map::Entry;

type Env = HashMap<SmartString, Option<LoxObject>>;

pub struct EnvironmentTree {
    global: GlobalEnv,
    leaves: Vec<Leaf>,
}

#[derive(Clone)]
enum Leaf {
    Node(Node),
    Closure(Closure),
}

impl Environment for Leaf {
    fn get(&self, key: &str) -> Result<Option<LoxObject>> {
        match self {
            Self::Node(node) => node.get(key),
            Self::Closure(closure) => closure.get(key),
        }
    }

    fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        match self {
            Self::Node(node) => node.define(key, value),
            Self::Closure(closure) => closure.define(key, value),
        }
    }

    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()> {
        match self {
            Self::Node(node) => node.assign(key, value),
            Self::Closure(closure) => closure.assign(key, value),
        }
    }
}

impl Default for EnvironmentTree {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct GlobalEnv(pub Env);

impl std::ops::Deref for GlobalEnv {
    type Target = Env;
    fn deref(&self) -> &Env {
        &self.0
    }
}

impl std::ops::DerefMut for GlobalEnv {
    fn deref_mut(&mut self) -> &mut Env {
        &mut self.0
    }
}

impl EnvironmentTree {
    pub fn add_scope(&mut self) {
        // using cur_node here causes borrowck issues
        match self.leaves.last_mut() {
            Some(cur) => *cur = Leaf::Node(Node::new_scoped(cur.clone())),
            None => {
                self.leaves
                    .push(Leaf::Node(Node::new_global(&mut self.global)));
            }
        }
    }

    pub fn add_function_scope(&mut self, closure: Closure) {
        self.leaves.push(Leaf::Closure(closure));
    }

    fn cur_node(&mut self) -> Option<&mut Leaf> {
        self.leaves.last_mut()
    }

    fn cur_borrow(&self) -> Option<&Leaf> {
        self.leaves.last()
    }

    pub fn remove_scope(&mut self) {
        let leaf = self.cur_node().expect("tried to remove Global Scope");

        match leaf {
            Leaf::Node(node) => {
                if let Parent::Scoped(parent) = &mut node.parent {
                    *node = *parent.clone();
                } else {
                    self.leaves.pop();
                }
            }

            Leaf::Closure(closure) => {
                if let Some(parent) = &mut closure.parent {
                    *closure = *parent.clone()
                } else {
                    self.leaves.pop();
                }
            }
        }
    }

    pub fn cur_leaf(&self) -> &dyn Environment {
        let Some(last) = self.leaves.last() else {
            return &self.global;
        };
        last
    }

    pub fn cur_mut(&mut self) -> &mut dyn Environment {
        let Some(last) = self.leaves.last_mut() else {
            return &mut self.global;
        };
        last
    }

    fn new() -> Self {
        Self {
            global: GlobalEnv(Env::new()),
            leaves: Vec::new(),
        }
    }

    pub fn new_closure(&self) -> Closure {
        if let Some(last) = self.leaves.last() {
            Closure::from(last)
        } else {
            Closure::from(&self.global)
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    inner: Env,
    parent: Parent,
}

#[derive(Clone, Debug)]
enum Parent {
    Global(NonNull<GlobalEnv>),
    Scoped(Box<Node>),
}

#[derive(Clone, Debug)]
pub struct Closure {
    env: Env,
    parent: Option<Box<Closure>>,
}

impl From<&Leaf> for Closure {
    fn from(leaf: &Leaf) -> Self {
        match leaf {
            Leaf::Closure(closure) => closure.clone(),
            Leaf::Node(node) => node.into(),
        }
    }
}

impl From<&GlobalEnv> for Closure {
    fn from(env: &GlobalEnv) -> Self {
        Self {
            env: env.0.clone(),
            parent: None,
        }
    }
}

impl From<&Node> for Closure {
    fn from(node: &Node) -> Self {
        let parent: Closure = match node.parent {
            Parent::Global(global) => unsafe { global.as_ref() }.into(),
            Parent::Scoped(ref node) => node.as_ref().into(),
        };
        let parent = Some(Box::new(parent));
        Self {
            env: node.inner.clone(),
            parent,
        }
    }
}

pub trait Environment {
    fn get(&self, key: &str) -> Result<Option<LoxObject>>;
    fn define(&mut self, key: SmartString, value: Option<LoxObject>);
    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()>;
}

impl Environment for Node {
    fn get(&self, key: &str) -> Result<Option<LoxObject>> {
        if let Some(value) = self.inner.get(key) {
            Ok(value.clone())
        } else {
            match &self.parent {
                Parent::Scoped(outer) => outer.get(key),
                // SAFETY: valid because new Envs are only created through Nodes or EnvironmenTree, which both create valid types
                // can't be mutated because we'll be accessing this through EnvironmentTree, which
                // can only mutate Global if it's exclusive, there are no other threads beacuse
                // we're not Sync,
                // edge case to consider:
                // assign("val", val) if val is in global scope
                // it's okay because we return a copy, ensuring that the reference doesn't exist
                Parent::Global(environment) => unsafe { environment.as_ref() }.get(key),
            }
        }
    }

    fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        self.inner.insert(key, value);
    }

    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()> {
        if let Entry::Occupied(mut variable) = self.inner.entry(key.clone()) {
            *variable.get_mut() = Some(value);
            Ok(())
        } else {
            match &mut self.parent {
                // SAFETY: see get() for validity/alignment safety
                // Other references can only be created through this function or through EnvironmentTree,
                // which is borrowed mutably for assign_global to be called, so we're safe
                Parent::Global(g) => unsafe { g.as_mut() }.assign(key, value),
                Parent::Scoped(outer) => outer.assign(key, value),
            }
        }
    }
}

impl Environment for Closure {
    fn get(&self, key: &str) -> Result<Option<LoxObject>> {
        if let Some(value) = self.env.get(key) {
            Ok(value.clone())
        } else {
            match &self.parent {
                Some(closure) => closure.get(key),
                None => Err(RuntimeError::Undefined(key.into())),
            }
        }
    }

    fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        self.env.insert(key, value);
    }

    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()> {
        if let Entry::Occupied(mut variable) = self.env.entry(key.clone()) {
            *variable.get_mut() = Some(value);
            Ok(())
        } else {
            match &mut self.parent {
                Some(closure) => closure.assign(key, value),
                None => Err(RuntimeError::Undefined(key.into())),
            }
        }
    }
}

impl Environment for GlobalEnv {
    fn get(&self, key: &str) -> Result<Option<LoxObject>> {
        if let Some(value) = self.0.get(key) {
            Ok(value.clone())
        } else {
            Err(RuntimeError::Undefined(SmartString::from(key)))
        }
    }

    fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        self.0.insert(key, value);
    }

    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut variable) => {
                *variable.get_mut() = Some(value);
                Ok(())
            }
            Entry::Vacant(_) => Err(RuntimeError::Undefined(key)),
        }
    }
}

impl Node {
    fn new_global(env: &mut GlobalEnv) -> Self {
        Self {
            inner: Env::new(),
            parent: Parent::Global(env.into()),
        }
    }

    fn new_scoped(parent: Node) -> Self {
        Self {
            inner: Env::new(),
            parent: Parent::Scoped(Box::new(parent)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn global_scope() -> Result<()> {
        let mut env = EnvironmentTree::default();
        env.cur_mut()
            .define("a".into(), Some(LoxObject::Float(3.0)));
        assert_eq!(env.cur_leaf().get("a")?, Some(LoxObject::Float(3.0)));

        env.cur_mut()
            .assign("a".into(), LoxObject::String("foo".into()))?;
        assert_eq!(
            env.cur_leaf().get("a")?,
            Some(LoxObject::String("foo".into()))
        );

        Ok(())
    }
}
