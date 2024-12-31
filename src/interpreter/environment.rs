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
    leaves: Vec<LeafNode>,
}

impl fmt::Display for EnvironmentTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut level = 0;
        if let Some(LeafNode::Scoped(mut cur_node)) = self.cur_borrow().cloned() {
            while let Parent::Scoped(parent) = cur_node.parent {
                level += 1;
                cur_node = *parent;
            }
        }
        let environment = self.leaves.len();
        write!(
            f,
            "level {}, {} environment, {:?}",
            level,
            environment,
            self.cur_borrow()
        )
    }
}

impl Default for EnvironmentTree {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct GlobalEnv(Env);

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
            Some(cur) => match cur {
                LeafNode::Scoped(node) => *cur = LeafNode::new_scoped(node.clone()),
                LeafNode::Global => *cur = LeafNode::new_global_child(&mut self.global),
            },
            None => {
                eprintln!("--new global child");
                self.leaves
                    .push(LeafNode::new_global_child(&mut self.global));
            }
        }
    }

    pub fn add_function_scope(&mut self) {
        self.leaves
            .push(LeafNode::new_global_child(&mut self.global));
    }

    fn cur_node(&mut self) -> Option<&mut LeafNode> {
        self.leaves.last_mut()
    }

    fn cur_borrow(&self) -> Option<&LeafNode> {
        self.leaves.last()
    }

    pub fn remove_scope(&mut self) {
        // this gets kind of complicated so I'm gonna comment this
        // this is a reference to the slot in the vec
        let leaf_slot = self.cur_node().expect("tried to remove Global Scope");
        let LeafNode::Scoped(node) = leaf_slot else {
            // if the leaf in the slot is LeafNode::Global, that means removing the scope will just
            // delete it, so we remove it from the Leaves list
            self.leaves.pop();
            return;
        };

        // otherwise, if it's parent is local then set itself to its parent, if it's parent is
        // global, then we set it to that placeholder
        if let Parent::Scoped(parent) = &mut node.parent {
            *node = *parent.clone();
        } else {
            *leaf_slot = LeafNode::Global
        }
    }

    pub fn cur_leaf(&self) -> &dyn Environment {
        let Some(last) = self.leaves.last() else {
            return &self.global;
        };
        match last {
            LeafNode::Scoped(node) => node,
            LeafNode::Global => &self.global,
        }
    }

    pub fn cur_mut(&mut self) -> &mut dyn Environment {
        let Some(last) = self.leaves.last_mut() else {
            return &mut self.global;
        };
        if let LeafNode::Scoped(node) = last {
            node
        } else {
            //borrow checker was weird with a match statement so I had to do an if let
            &mut self.global
        }
    }

    fn new() -> Self {
        Self {
            global: GlobalEnv(Env::new()),
            leaves: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum LeafNode {
    Scoped(Node),
    Global,
}

impl LeafNode {
    fn new_scoped(node: Node) -> Self {
        Self::Scoped(Node::new_scoped(node))
    }

    /// Create a new node whose parent is the global environment
    fn new_global_child(env: &mut GlobalEnv) -> Self {
        Self::Scoped(Node::new_global(env))
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

impl Environment for GlobalEnv {
    fn get(&self, key: &str) -> Result<Option<LoxObject>> {
        println!("global get {:?}", self);
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
