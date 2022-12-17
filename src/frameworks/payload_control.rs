use crate::HeaderMap;
use std::any::Any;
use std::fmt::Debug;

pub trait PayloadControl {
    const MAX_PAYLOAD_SIZE: Option<usize>;
    const BUFFER_CAPACITY: Option<usize>;
}

pub trait DebuggableAny: Debug + Any {}
