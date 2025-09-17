// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

mod error;
mod functions;
mod schema;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub use error::Error;
pub use functions::*;
pub use schema::{CourseUsers, UserCourses};

/// Course Access Contract
///
/// This contract manages user access to courses in the SkillCert platform.
/// It provides functionality to grant, revoke, and query course access permissions,
/// as well as manage user profiles.
#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// One-time constructor to set owner and config addresses.
    pub fn Initialize(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::Initialize(env, caller, user_mgmt_addr, course_registry_addr)
    }

    /// Grant access to a specific user for a given course.
    pub fn GrantAccess(env: Env, course_id: String, user: Address) {
        functions::grant_access::GrantAccess(env, course_id, user)
    }

    /// Revoke access for a specific user from a course.
    pub fn RevokeAccess(env: Env, course_id: String, user: Address) -> bool {
        functions::revoke_access::RevokeAccess(env, course_id, user)
    }

    /// Save or update a user's profile on-chain.
    pub fn SaveProfile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
    ) {
        let user = env.current_contract_address();
        functions::save_profile::SaveProfile(env, name, email, profession, goals, country, user);
    }

    /// List all courses a user has access to.
    pub fn ListUserCourses(env: Env, user: Address) -> UserCourses {
        functions::list_user_courses::ListUserCourses(env, user)
    }

    /// List all users who have access to a course.
    pub fn ListCourseAccess(env: Env, course_id: String) -> CourseUsers {
        functions::list_course_access::ListCourseAccess(env, course_id)
    }

    /// Revoke all user access for a course.
    pub fn RevokeAllAccess(env: Env, user: Address, course_id: String) -> u32 {
        functions::revoke_all_access::RevokeAllAccess(env, user, course_id)
    }

    /// Configure external contract addresses used for auth checks.
    pub fn SetConfig(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::SetContractAddrs(env, caller, user_mgmt_addr, course_registry_addr)
    }
}
