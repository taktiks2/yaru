pub mod aggregate;
pub mod events;
pub mod repository;
pub mod specification;
pub mod value_objects;

pub use aggregate::TaskAggregate;
pub use events::{
    DomainEvent, TaskCompleted, TaskCreated, TaskTagAdded, TaskTagRemoved, TaskTitleChanged,
};
pub use repository::TaskRepository;
pub use specification::{
    AndSpecification, OrSpecification, TaskById, TaskByPriority, TaskByStatus, TaskByTag,
    TaskOverdue, TaskSpecification,
};
pub use value_objects::{DueDate, Priority, Status, TaskDescription, TaskId, TaskTitle};
