pub mod task {
    mod add;
    mod delete;
    mod edit;
    mod list;
    mod show;

    pub use add::{AddTaskParams, add_task};
    pub use delete::{DeleteTaskParams, delete_task};
    pub use edit::{EditTaskParams, edit_task};
    pub use list::{ListTasksParams, list_tasks};
    pub use show::{ShowTaskParams, show_task};
}

pub mod tag {
    mod add;
    mod delete;
    mod edit;
    mod list;
    mod show;

    pub use add::{AddTagParams, add_tag};
    pub use delete::{DeleteTagParams, delete_tag};
    pub use edit::{EditTagParams, edit_tag};
    pub use list::list_tags;
    pub use show::{ShowTagParams, show_tag};
}
