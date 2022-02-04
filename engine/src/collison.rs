/*
thing A hits thing B
collision layers because everything is an entity
one layer for the things that can interact

part of entity then needs a collision layer 

search and calling abstract methods 

1. check collsion layer
2. check if collison has occured 
3. check every object on layer 
 -- check for object A we care about, then all B
 -- flag some entities for check for collsion, don't care if enemies hit each other
 -- check if bullet hits enemy; enemy hits player
4. Call on_collison which is part of entity 

**trait on_collsion ** -> then implemented in game
match on enum projectile/entity type but only care about projectile and enemy and player
-- player: deletes self and what it hit + increase score
-- enemy: ends game

*/
