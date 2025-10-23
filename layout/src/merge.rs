use super::{Bind, Layout};
use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;

impl Layout {
    pub fn merge(&mut self, op: &(impl LayoutOp + Debug + ?Sized), mut rhs: Self) {
        op.merge(self, &mut rhs);
        if let Some(rchildren) = rhs.children {
            if let Some(children) = &mut self.children {
                let children = children
                    .iter_mut()
                    .zip_longest(rchildren)
                    .map(|x| match x {
                        Both(l, r) => {
                            l.merge(op, r);
                            l.clone()
                        }
                        Left(l) => l.clone(),
                        Right(r) => r,
                    })
                    .collect();
                self.children = Some(children);
            } else {
                self.children = Some(rchildren);
            }
        }
    }
}

pub trait LayoutOp: Debug {
    fn merge_value(&self, l: &mut Value, r: &Value) -> Option<Value>;
    fn merge(&self, lhs: &mut Layout, rhs: &mut Layout) {
        lhs.bind = match (&mut lhs.bind, &mut rhs.bind) {
            (Some(l), Some(r)) => {
                let nv = l
                    .into_iter()
                    .chain(r)
                    .fold(HashMap::new(), |mut m, (k, v)| {
                        m.entry(k.to_owned())
                            .and_modify(|old: &mut Bind| {
                                let nd = match (&mut old.default, &v.default) {
                                    (Some(x), Some(y)) => self.merge_value(x, y),
                                    (Some(x), None) => Some(x.clone()),
                                    (None, Some(y)) => Some(y.clone()),
                                    (None, None) => None,
                                };
                                old.default = nd;
                            })
                            .or_insert(v.to_owned());
                        m
                    });
                Some(nv)
            }
            (Some(l), None) => Some(l.to_owned()),
            (None, Some(y)) => Some(y.to_owned()),
            (None, None) => None,
        };
    }
}

#[derive(Debug)]
pub struct Concat;
impl LayoutOp for Concat {
    fn merge_value(&self, x: &mut Value, y: &Value) -> Option<Value> {
        let n = match (x, y) {
            (Value::Number(x), Value::Number(r)) => {
                json!(x.as_f64().unwrap() + r.as_f64().unwrap())
            }
            (Value::Bool(x), Value::Bool(r)) => {
                json!(*x || *r)
            }
            (Value::String(x), Value::String(r)) => {
                x.push_str(&r);
                json!(x)
            }
            (Value::Object(x), Value::Object(r)) => {
                for (k, v) in r {
                    x.entry(k)
                        .and_modify(|x| *x = v.clone())
                        .or_insert_with(|| v.clone());
                }
                json!(x)
            }
            (Value::Array(x), Value::Array(r)) => {
                json!([x.clone(), r.clone()].concat())
            }
            _ => y.clone(),
        };
        Some(n)
    }
}

#[derive(Debug)]
pub struct Delete;
impl LayoutOp for Delete {
    fn merge_value(&self, x: &mut Value, y: &Value) -> Option<Value> {
        let n = match (x, y) {
            (Value::Number(x), Value::Number(r)) => {
                json!(x.as_f64().unwrap() - r.as_f64().unwrap())
            }
            (Value::Bool(x), Value::Bool(r)) => {
                json!(*x && *r)
            }
            (Value::String(x), Value::String(r)) => {
                json!(x.replace(r, ""))
            }
            (Value::String(x), Value::Number(r)) => {
                let l = x.len();
                let s = r.as_u64().unwrap() as usize;
                let e = if s >= l { 0 } else { l.saturating_sub(s) };
                json!(x[..e])
            }
            (Value::Object(x), Value::Object(r)) => {
                for (k, _v) in r {
                    if x.contains_key(k) {
                        x.remove(k);
                    };
                }
                json!(x)
            }
            (Value::Array(x), Value::Array(_r)) => {
                json!(x)
            }
            _ => y.clone(),
        };
        Some(n)
    }
}

#[derive(Debug)]
pub struct Replace;
impl LayoutOp for Replace {
    fn merge_value(&self, x: &mut Value, r: &Value) -> Option<Value> {
        let y = match (x, r) {
            (Value::Number(_x), Value::Number(r)) => {
                json!(r.as_f64().unwrap())
            }
            (Value::Bool(_x), Value::Bool(r)) => {
                json!(*r)
            }
            (Value::String(_x), Value::String(r)) => {
                json!(r)
            }
            (Value::Object(_x), Value::Object(r)) => {
                json!(r)
            }
            (Value::Array(_x), Value::Array(r)) => {
                json!(r)
            }
            _ => r.clone(),
        };
        Some(y)
    }
}
