use poise::{serenity_prelude as serenity, FrameworkContext};

use crate::{common::Data, error::CommandError};

pub struct EventInfo<'a> {
    pub ctx: serenity::Context,
    pub event: serenity::FullEvent,
    pub framework: FrameworkContext<'a, Data, CommandError>,
    pub data: Data,
}
