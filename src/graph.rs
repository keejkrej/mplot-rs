use crate::boxplot::BoxplotData;
use crate::curve::CurveData;

#[derive(Clone, Debug)]
pub enum GraphEntity {
    Curve(CurveData),
    Boxplot(BoxplotData),
}

/// Defines the trait used by Plot to add graph entities.
///
/// plotpy stores Python commands in a string buffer; mplot stores structured draw data instead.
pub trait GraphMaker {
    fn get_buffer(&self) -> &String;
    fn clear_buffer(&mut self);
    fn graph_entity(&self) -> Option<GraphEntity>;
}
