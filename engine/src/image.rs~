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
let NUM_LAYERS: usize = 10;

fn check_overlap_1D(low1: f32, high1: f32, low2: f32, high2: f32) -> bool {
	if low1 <= low2{
		return low2 < high1;
	}
	return low1 < high2;
}

fn check_overlap_2D(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
	let x_overlap = check_overlap_1D(pos1.x, pos1.x + size1.x, pos2.x, pos2.x + size2.x);
	let y_overlap = check_overlap_1D(pos1.y, pos2.y + size1.y, pos2.y, pos2.y + size2.y);
	return x_overlap && y_overlap;
}

fn check_collision_layer(this: entity::Entity, layer: Vec<entity::Entity>) {
	let this_pos = this.position + this.hurt_box.position;
	let this_size = this.hurt_box.size;
	for other in layer{
		if this == other { continue; }
		let other_pos = other.position + other.hurt_box.position;
		let other_size = other.hurt_box.size;
		if check_overlap_2D(this_pos, this_size, other_pos, other_size){
			this.on_collison(other);
		}
	}
}

fn split_layers(entities: Vec<entity::Entity>) -> [Vec<entity::Entity>; NUM_LAYERS]{
	let layers: [Vec<entity::Entity>; NUM_LAYERS] = [Vec::New(); NUM_LAYERS];
	for enity in entities{
		layers[entity.hurt_box.collision_layer].push(entity);
	}
	return layers;
}

fn check_collisions(entities: Vec<entity::Entity>){
	let layers = split_layers(entities);
	
	for entity in entities{
		if entity.check_collision{
			check_collision_layer(entity, layers[entity.hurt_box.collision_layer]);
		}
	}
}