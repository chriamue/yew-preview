use std::rc::Rc;
use yew::Html;

/// A runtime-typed argument value used for live prop editing.
#[derive(Clone, PartialEq, Debug)]
pub enum ArgValue {
    Bool(bool),
    Int(i64),
    /// Ranged integer rendered as a slider: (value, min, max)
    IntRange(i64, i64, i64),
    Float(f64),
    Text(String),
}

/// Holds the initial arg values and a render function that produces Html
/// from the current arg state. Used by `create_interactive_preview!`.
pub struct InteractiveArgs {
    pub values: Vec<(String, ArgValue)>,
    pub render_fn: Rc<dyn Fn(&[(String, ArgValue)]) -> Html>,
}

impl Clone for InteractiveArgs {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            render_fn: self.render_fn.clone(),
        }
    }
}

impl PartialEq for InteractiveArgs {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && Rc::ptr_eq(&self.render_fn, &other.render_fn)
    }
}

// --- Accessor helpers for use inside render closures ---

pub fn get_bool(args: &[(String, ArgValue)], key: &str) -> bool {
    args.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| if let ArgValue::Bool(b) = v { Some(*b) } else { None })
        .unwrap_or_default()
}

pub fn get_int(args: &[(String, ArgValue)], key: &str) -> i64 {
    args.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| match v {
            ArgValue::Int(i) => Some(*i),
            ArgValue::IntRange(i, _, _) => Some(*i),
            _ => None,
        })
        .unwrap_or_default()
}

pub fn get_float(args: &[(String, ArgValue)], key: &str) -> f64 {
    args.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| if let ArgValue::Float(f) = v { Some(*f) } else { None })
        .unwrap_or_default()
}

pub fn get_text(args: &[(String, ArgValue)], key: &str) -> String {
    args.iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| if let ArgValue::Text(s) = v { Some(s.clone()) } else { None })
        .unwrap_or_default()
}
