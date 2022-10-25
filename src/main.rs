#![allow(unused)]
use bevy::prelude::*;
// TcpListener listens for connections, Stream Enables
use std::net::{TcpListener, TcpStream}
use std::io::{Read, Write, Error}
use std::io::prelude::*;
use std::net::TcpStream;
use iyes_loopless::state::NextState;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// Component Markers ( may be unecessary )
#[derive(Component)]
struct Host;
#[derive(Component)]
struct Peer;

// GAMEWIDE CONSTANTS / RESOURCES
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum NetworkState{
	Start, // attempts to find active host
	SyncingTo, // host found, connecting to 
	BecomeHost, // is becoming the host
	// by this point, a host should exist
	FindingPeer, // host exists, is accepting a peer (LOOPS)
	BattlingHost, // Battling The Host (BLOCKS)
	BattlingPeer, // Battling The Peer (BLOCKS)
}

// keeping original Weapon resource in case
pub enum Weapon{
	Rock,
	Paper,
	Scissors,
}
// a Component now used in Bundle
#[derive(PartialEq, Debug, Component)]
pub enum WeaponComp{
	Rock,
	Paper,
	Scissors,
}

// Bundles hold the individual choices for what is being kept track of.
// This way, each client knows which one is what.
#[derive(Bundle)]
struct Host{
	_host: Host,
	choice: WeaponComp,
};
#[derive(Bundle)]
struct Peer{
	_peer: Peer,
	choice: WeaponComp,
};






fn main() {//-> std::io::Result<()> {
	App::new()
	.add_fixed_timestep(
		Duration::from_millis(250),
		// give it a label
		"update_wait",
	)
	.add_plugins(DefaultPlugins)
	// starting state, identifies if a host already exists, enters corresponding NetworkState
	.add_startup_system(startRPS)

	// dictating loopless states that only run once before moving to next State.
	.add_loopless_state(NetworkState::Start)
	.add_loopless_state(NetworkState::SyncingTo)
	.add_loopless_state(NetworkState::BecomeHost)
	
	// Add our various systems
	// On states Enter and Exit
	// when entering SyncingTo, connectToHost and start battling for both,
	.add_enter_system(NetworkState::SyncingTo, connectToHost)
	// when entering BecomeHost, initalize the host variables and then enter FindingPeer state
	.add_enter_system(NetworkState::BecomeHost, initializeHost)

	// findingPeer loops until peer is found, initializes the values for peer, then enters BattlingPeer
	.add_system_set(ConditionSet::new()
		.run_in_state(NetworkState::FindingPeer)
		.with_system(discoverPeer)
		.into()
	)
	// BattlingHost state refers to THIS Peer Client is fighting a Host client.
	.add_system_set(ConditionSet::new()
		.run_in_state(NetworkState::BattlingHost)
		.with_system(makeChoiceHost)
		.into()
	)
	// BattlingHPeer state refers to THIS Peer Client is fighting a Host client.
	.add_system_set(ConditionSet::new()
		.run_in_state(NetworkState::BattlingPeer)

		.with_system(makeChoiceHost)
		.into()
	)
	// should just close because if a host stops connecting there are no other hosts open
	.add_exit_system(NetworkState::FindingPeer, noClientsOpen)
	// battling host left, call repairHost and become the newHost (challenging)
	.add_exit_system(NetworkState::BattlingHost, repairHost)
	//  battling peer left, account for the dropped connection and await a new one.
	.add_exit_system(NetworkState::BattlingPeer, peerDisconnect)
	.run();
	//test_battle_logic();
}



/**
 *  Randomly generates a weapon. Each outcome has an equal opportunity to happen.
 */
impl Distribution<Weapon> for Standard {
	
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weapon {
    	match rng.gen_range(0..=2) {
            0 => Weapon::Rock,
            1 => Weapon::Paper,
            _ => Weapon::Scissors,
        }
    }
}

// needs to determine whether there is a host yet, decides whether to enter either state
 fn startRPS(mut commands: Commands){

 }

 /**
 * Runs 100 randomly generated battles to test the battle logic, should test that clients are connected first, 
 */
fn test_battle_logic(){
	for _i in 1..100 {
		battle(rand::random(), rand::random());
	};
	
}


/* Battles rps */
fn battle(my_weapon: Weapon, opp_weapon: Weapon){
	if my_weapon == Weapon::Rock {
		if opp_weapon == Weapon::Rock {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} is covered by {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} breaks {:?}", my_weapon, opp_weapon);
		}
	}
	if my_weapon == Weapon::Paper {
		if opp_weapon == Weapon::Rock {
			println!("{:?} covers {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} is cut by {:?}", my_weapon, opp_weapon);
		}
	}
	if my_weapon == Weapon::Scissors {
		if opp_weapon == Weapon::Rock {
			println!("{:?} is crushed by {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} cuts {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
	}
}