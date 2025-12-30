pub mod task {
    mod add;
    mod delete;
    mod list;
    mod show;

    pub use add::add_task;
    pub use delete::delete_task;
    pub use list::list_tasks;
    pub use show::show_task;
}

pub mod tag {
    mod add;
    mod delete;
    mod list;
    mod show;

    pub use add::add_tag;
    pub use delete::delete_tag;
    pub use list::list_tags;
    pub use show::show_tag;
}
