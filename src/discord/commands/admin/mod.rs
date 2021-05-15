use serenity::framework::standard::macros::group;

mod loadmenus;
use loadmenus::*;

#[group]
#[commands(loadmenus)]
pub struct Admin;


