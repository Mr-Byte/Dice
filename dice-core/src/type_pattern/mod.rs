use crate::value::Class;
use std::collections::HashMap;
use std::rc::Rc;

pub enum TypeOrdering {
    None,
    Sub,
    Self_,
    Super,
}

pub trait TypeOrd<Rhs: ?Sized = Self> {
    fn compare(&self, other: &Rhs) -> TypeOrdering;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum PatternPart {
    Receiver,
    Null,
    Class { class: Class, is_nullable: bool },
}

pub struct TypePattern {
    parts: Rc<[PatternPart]>,
}

#[derive(Default)]
pub struct TypePatternTree<T> {
    root: TypePatternNode<T>,
}

impl<T> TypePatternTree<T> {
    pub fn insert(&mut self, pattern: TypePattern, value: T) {
        if pattern.parts.len() > 0 {
            let mut current_node = &mut self.root;

            for part in &*pattern.parts {
                if !current_node.children.contains_key(part) {
                    let new_node = Default::default();
                    current_node.children.insert(part.clone(), new_node);
                    current_node = current_node
                }

                current_node = current_node.children.get_mut(part).unwrap();
            }

            if current_node.value.is_none() {
                current_node.value = Some(Box::new(value))
            } else {
                todo!("Value already exists error.")
            }
        } else {
            panic!("Pattern is empty.")
        }
    }

    pub fn find_candidates(&mut self, pattern: TypePattern) -> Vec<&T> {
        // TODO: Find a list of best possible candidates, sorted by most likely match.
        todo!()
    }
}

pub struct TypePatternNode<T> {
    children: HashMap<PatternPart, TypePatternNode<T>>,
    value: Option<Box<T>>,
}

impl<T> Default for TypePatternNode<T> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            value: None,
        }
    }
}

impl TypeOrd for Class {
    fn compare(&self, other: &Self) -> TypeOrdering {
        if self.instance_type_id() == other.instance_type_id() {
            TypeOrdering::Self_
        } else if self.is_derived_from(other) {
            TypeOrdering::Sub
        } else if other.is_derived_from(self) {
            TypeOrdering::Super
        } else {
            TypeOrdering::None
        }
    }
}
