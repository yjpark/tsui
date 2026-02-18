use facet::Shape;
use facet_value::Value;

pub trait Model {
    const SHAPE: &'static Shape;

    fn get_value(&self) -> Value;

    fn get_field(&self, key: &str) -> Option<Value>;
    fn set_field(&self, key: &str, value: Value) -> Option<Value>;
}
