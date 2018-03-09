
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
this would now create an aniaml with species dog

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

