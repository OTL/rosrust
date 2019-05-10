use crate::goal_id_generator::GoalIdGenerator;
use crate::msg::actionlib_msgs::{GoalID, GoalStatus};
use crate::Goal;
use rosrust;

pub struct StatusTracker<T> {
    pub goal: Option<Goal<T>>,
    pub status: GoalStatus,
    pub handle_destruction_time: rosrust::Time,
    pub id_generator: GoalIdGenerator,
}

impl<T: rosrust::Message> StatusTracker<T> {
    pub fn from_status(goal_id: GoalID, status: u8) -> Self {
        let goal = None;
        let id_generator = GoalIdGenerator::new();

        let status = GoalStatus {
            goal_id,
            status,
            text: String::new(),
        };

        let handle_destruction_time = rosrust::Time::default();
        Self {
            goal,
            status,
            handle_destruction_time,
            id_generator,
        }
    }

    pub fn from_goal(goal: Goal<T>) -> Self {
        let id_generator = GoalIdGenerator::new();

        let mut goal_id = if goal.id.id == "" {
            id_generator.generate_id()
        } else {
            goal.id.clone()
        };

        if goal_id.stamp.nanos() == 0 {
            goal_id.stamp = rosrust::now()
        }

        let status = GoalStatus {
            goal_id,
            status: GoalStatus::PENDING,
            text: String::new(),
        };

        let handle_destruction_time = rosrust::Time::default();
        Self {
            goal: Some(goal),
            status,
            handle_destruction_time,
            id_generator,
        }
    }
}
