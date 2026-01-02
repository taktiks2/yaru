pub mod task {
    mod add;
    mod delete;
    mod edit;
    mod list;
    mod show;

    pub use add::{add_task, AddTaskParams};
    pub use delete::{delete_task, DeleteTaskParams};
    pub use edit::{edit_task, EditTaskParams};
    pub use list::{list_tasks, ListTasksParams};
    pub use show::{show_task, ShowTaskParams};
}

pub mod tag {
    mod add;
    mod delete;
    mod edit;
    mod list;
    mod show;

    pub use add::{add_tag, AddTagParams};
    pub use delete::{delete_tag, DeleteTagParams};
    pub use edit::{edit_tag, EditTagParams};
    pub use list::list_tags;
    pub use show::{show_tag, ShowTagParams};
}
