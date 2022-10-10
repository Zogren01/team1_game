# CS1666 Team1 Management weeks

1. 09/14 - 09/20
	* Manager: Zach Ogren
	* Goals:
		1. Setup game repo
		2. Add team description
2. 09/28 - 10/04
	* Manager: Brian Sostek
	* Goals:
		1. Create game credits
2. 10/05 - 10/11
	* Manager: NAME
	* Goals:
		1. Basic game layout
		2. Character with simple movement
		3. Collision detection
		4. Create first room (simple loading area)
		5. Develop line of sight (stationary)
2. 10/12 - 10/18
	* Manager: James Baker
	* Goals:
		1. First main room
		2. Map Hazards
		3. Moving enemies with line of sight
		4. Have some barrels and other breakable object sprites 

2. 10/19 - 10/25
	* Manager: Ethan Zhao
	* Goals:
		1. Enemies can pathfind with A* algorithm to reach set goal
		1. 
		1. Enemies can be killed
		1. Have targetable/breakable objects implemented
			- Breakable spiderweb (negative vector on  mvoement speed)
2. 10/26 - 11/01
	* Manager: NAME
	* Goals:
		1. Enemies use their line of sight to determine what their target for A* pathfinding
		1. Explosion vectors when a sprite is close to the barrel when shot
		1. 	1. Melee enemy:
		* prioritizes damaging the player with varying levels of aggression based on their own health
		* will attempt to damage player with surrounding obstacles if possible
		* prioritize taking the player somemplace dangerous while retreating if possible
		* if it encounters a ranged enemy, it will ignore its health and continually attempt to keep itself between the player and the ranged enemy
	2. Ranged enemy:
		* prioritizes keeping the maximum distance between itself and the player that is within the range of its attack
		* if it encounters a melee enemy, it will attempt to keep itself within this enemies line of sight while attacking the player
		* if it's health is low enough, it will search for a health enemy
	3. Health enemy:
		* this enemy will run for cover on sight of the player
		* this behavior will be overwritten if it encounters an enemy that does not have full health, in which case it will try to heal this enemy using its melee attack
		* will make a dash towards the player if the player is at low enough health
2. 11/02 - 11/08
	* Manager: Jacob Salmon
	* Goals:
		1. Have shrapnel from barrels working (damaging enemies and player)
		1. ...
		1. ...
2. 11/09 - 11/15
	* Manager: Bailey Mathien
	* Goals:
		1. Have barrel shrapnel interact  with other breakable/interactable objects
		1. Implement item - Jetpack & Umbrella
		1. ...
2. 11/16 - 11/29
	* Manager: NAME
	* Goals:
		1. Implement item - Jumping Boots
		1. ...
		1. ...
2. 11/30 - 12/06
	* Manager: Giovanni Versace
	* Goals:
		1. ...
		1. ...
		1. ...		
