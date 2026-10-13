//! What every Nixe app shares underneath its window: who it is, where its
//! files live, its preferences file, and running other programs.

pub mod app;
pub mod cmd;
pub mod migrate;
pub mod paths;
pub mod prefs;
