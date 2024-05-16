#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env, token, String, Vec };

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    Project(u64),
    Milestone(u64, u64), // Project ID, Milestone ID
    CurrentProjectId,
}

#[derive(Clone)]
#[contracttype]
pub struct Project {
    organizer: Address,
    description: String,
    goal_amount: u64,
    total_funds: u64,
    completed: bool,
    milestones: Vec<Milestone>,
}

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
    description: String,
    amount: u64,
    completed: bool,
}

#[contract]
pub struct CrowdfundedEnvironmentalProjects;

#[contractimpl]
impl CrowdfundedEnvironmentalProjects {
    pub fn create_project(env: Env, from: Address, description: String, goal_amount: u64, initial_milestones: Vec<Milestone>) {
        from.require_auth();
        let mut project_id = env.storage().instance().get(&StorageKey::CurrentProjectId).unwrap();

        let project = Project {
            organizer: from,
            description,
            goal_amount,
            total_funds: 0,
            completed: false,
            milestones: initial_milestones,
        };

        project_id = project_id + 1;

        env.storage().instance().set(&StorageKey::Project(project_id), &project);
        env.storage().instance().set(&StorageKey::CurrentProjectId, &project_id);
    }

    pub fn donate_funds(env: Env, from: Address, token: Address, amount: i128, project_id: u64) {
        from.require_auth();
        let project: Project = env
            .storage()
            .instance()
            .get(&StorageKey::Project(project_id))
            .unwrap();

        token::Client::new(&env, &token).transfer(&from, &project.organizer, &amount);

        let mut updated_project = project;
        updated_project.total_funds += amount as u64;
        env.storage().instance().set(&StorageKey::Project(project_id), &updated_project);
    }

    pub fn create_milestone(env: Env, project_id: u64, description: String, amount: u64) {
        let mut milestone_id = env
            .storage()
            .instance()
            .get(&StorageKey::Milestone(project_id, 0))
            .unwrap();

        let milestone = Milestone {
            description,
            amount,
            completed: false,
        };

        milestone_id = milestone_id + 1;

        env.storage().instance().set(&StorageKey::Milestone(project_id, milestone_id), &milestone);
    }

    pub fn complete_milestone(env: Env, project_id: u64, milestone_id: u64) {
        let mut project: Project = env
            .storage()
            .instance()
            .get(&StorageKey::Project(project_id))
            .unwrap();
        let milestone: Milestone = env
            .storage()
            .instance()
            .get(&StorageKey::Milestone(project_id, milestone_id))
            .unwrap();

        project.total_funds -= milestone.amount;

        let mut updated_milestone = milestone;
        updated_milestone.completed = true;

        env.storage().instance().set(&StorageKey::Project(project_id), &project);
        env.storage()
            .instance()
            .set(&StorageKey::Milestone(project_id, milestone_id), &updated_milestone);
    }
}
