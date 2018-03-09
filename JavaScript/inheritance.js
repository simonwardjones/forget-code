
// Prototypes

// Creating an object can be done by defining the constructor:
function Animal(age,species){
  this.age = age;
  this.species = species;
}
// in the above this refers to the object being constructed!
let ben = new Animal(13,'dog')

console.log(ben)
// when we look at ben he has two proerties and a proto with the animal constructor

function Dog(age){
	Animal.call(this,age,'dog')
	// call the animal function with given context
}
// this would now create an aniaml with species dog

// Now lets use and example with prototype
function Animal(age,species,sound){
  this.age = age;
  this.species = species;
  this.sound = sound;
}
Animal.prototype.speak = function(){
	return this.sound
}

// Now I need to re work Dog:

function Dog(age){
	Animal.call(this,age,'dog','woof')
}
Dog.prototype = Object.create(Animal.prototype)
Dog.prototype.constructor = Dog
// This fixes the constructor being wrong


// Really it comes down to two things
// - fix the prototype
// - fix the constructor

// question why not use
// Dog.prototype = Animal.prototype??
// because then they are the same object !!!
// and if we add a dog method to it all animals will have it!

// why not use 
// Dog.prototype = new Animal()
// well if you look in the prototype of dog it includes all the instance specific
// variables again but they are undefined as we didnt pass them to 
// the Animal constructor. Just messy and could create strange bug