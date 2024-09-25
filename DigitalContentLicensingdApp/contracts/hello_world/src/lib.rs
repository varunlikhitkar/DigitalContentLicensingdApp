#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Struct to store content license information
#[contracttype]
#[derive(Clone)]
pub struct License {
    pub content_id: u64,       // Unique ID for the digital content
    pub creator: String,       // Creator's name or ID
    pub license_fee: u64,      // Fee for licensing the content
    pub licensee: String,      // Licensee's name or ID
    pub license_approved: bool, // Status of license approval
}

// Mapping for storing the license information
#[contracttype]
pub enum LicenseBook {
    License(u64),
}

// Constant for tracking license count
const COUNT_LICENSE: Symbol = symbol_short!("C_LICENSE");

#[contract]
pub struct ContentLicenseContract;

#[contractimpl]
impl ContentLicenseContract {
    // Function to create a new license for digital content
    pub fn create_license(env: Env, creator: String, license_fee: u64) -> u64 {
        let mut count_license: u64 = env.storage().instance().get(&COUNT_LICENSE).unwrap_or(0);
        count_license += 1;

        let new_license = License {
            content_id: count_license,
            creator: creator.clone(),
            license_fee,
            licensee: String::from_str(&env, "Not_Assigned"),
            license_approved: false,
        };

        env.storage().instance().set(&LicenseBook::License(count_license.clone()), &new_license);
        env.storage().instance().set(&COUNT_LICENSE, &count_license);

        log!(&env, "License Created for Content ID: {}, by Creator: {}", count_license, creator);
        count_license
    }

    // Function to approve and assign a license to a licensee
    pub fn approve_license(env: Env, content_id: u64, licensee: String) {
        let mut license = Self::get_license_by_id(env.clone(), content_id.clone());

        if license.license_approved == false {
            license.licensee = licensee.clone();
            license.license_approved = true;

            env.storage().instance().set(&LicenseBook::License(content_id), &license);
            log!(&env, "License for Content ID: {}, approved for Licensee: {}", content_id, licensee);
        } else {
            log!(&env, "License already approved!");
            panic!("License already approved!");
        }
    }

    // Helper function to retrieve license details by content ID
    pub fn get_license_by_id(env: Env, content_id: u64) -> License {
        let key = LicenseBook::License(content_id.clone());
        env.storage().instance().get(&key).unwrap_or(License {
            content_id: 0,
            creator: String::from_str(&env, "Not_Found"),
            license_fee: 0,
            licensee: String::from_str(&env, "Not_Found"),
            license_approved: false,
        })
    }
}
