mod functions;
mod schema;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub use functions::*;
pub use schema::{CourseUsers, UserCourses};

#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// Grant access to a specific user for a given course
    pub fn GrantAccess(env: Env, course_id: String, user: Address) {
        CourseAccessGrantAccess(env, course_id, user)
    }

    /// Revoke access for a specific user from a course
    pub fn RevokeAccess(env: Env, course_id: String, user: Address) -> bool {
        CourseAccessRevokeAccess(env, course_id, user)
    }

    /// Save or update a user's profile on-chain
    pub fn SaveProfile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
    ) {
        let user = env.current_contract_address();
        SaveProfile(env, name, email, profession, goals, country, user);
    }

    /// List all courses a user has access to
    pub fn ListUserCourses(env: Env, user: Address) -> UserCourses {
        CourseAccessListUserCourses(env, user)
    }

    /// List all users who have access to a course
    pub fn ListCourseAccess(env: Env, course_id: String) -> CourseUsers {
        CourseAccessListCourseAccess(env, course_id)
    }
}
