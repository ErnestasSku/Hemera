use bevy_ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct StartupSchedule;

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct UpdateSchedule;

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct AfterRenderSchedule;
