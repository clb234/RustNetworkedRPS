#[warn(unused_imports)]
use std::net::*;

#[derive(PartialEq, Debug)]
pub enum Weapon{
	Rock,
	Paper,
	Scissors
}

fn main(){
	battle(Weapon::Rock, Weapon::Paper);
}

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