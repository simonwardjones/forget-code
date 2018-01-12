// Introduction

// Check this out https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Introduction






// Grammer and types

// a one line comment
 
/* this is a longer, 
   multi-line comment
 */
 
/* You can't, however,  nest multiline comments */ 

// Declerations

var a;
console.log('The value of a is ' + a); // The value of a is undefined
// Hoisting means you can define the var anywhere in the function or statement (do it at the top...)

console.log('The value of b is ' + b); // The value of b is undefined
var b;

// You can't hoist a let or const variable
let x;
console.log('The value of x is ' + x); // The value of x is undefined

//function hoisting works

foo(); // "bar"

function foo() {
  console.log('bar');
}

/* Function expression */

console.log(baz); // undefined as you cant hoist a function expression 

var baz = function() {
  console.log('bar2');
};


// Literals

// array literal
var coffees = ['French Roast', 'Colombian', 'Kona'];

//bool
const truthlyness = true;

//numerical
var numerical = 8;

// Object literals
var sales = 'Toyota';

function carTypes(name) {
  if (name === 'Honda') {
    return name;
  } else {
    return "Sorry, we don't sell " + name + ".";
  }
}

var car = { myCar: 'Saturn', getCar: carTypes('Honda'), special: sales };

console.log(car.myCar);   // Saturn
console.log(car.getCar);  // Honda
console.log(car.special); // Toyota

// String literals
var words = 'one line \n another line'
// String interpolation with back tick
var name = 'Bob', time = 'today';
`Hello ${name}, how are you ${time}?`



// Control flow
// Conditionals
// if else

if (true) {
	console.log("Well that was true")
} else if (false) {
	console.log('Nope')
} else {
	console.log('maybe this?')
}

let  fruittype = 'Bananas'
// switch
switch (fruittype) {
  case 'Oranges':
    console.log('Oranges are $0.59 a pound.');
    break;
  case 'Apples':
    console.log('Apples are $0.32 a pound.');
    break;
  case 'Bananas':
    console.log('Bananas are $0.48 a pound.');
    break;
  case 'Cherries':
    console.log('Cherries are $3.00 a pound.');
    break;
  case 'Mangoes':
    console.log('Mangoes are $0.56 a pound.');
    break;
  case 'Papayas':
    console.log('Mangoes and papayas are $2.79 a pound.');
    break;
  default:
   console.log('Sorry, we are out of ' + fruittype + '.');
}
console.log("Is there anything else you'd like?");


// Exception handling

// throw fruittype; // throws an error (you can throw most things)
// throw {toString: function() { return "I'm an object!"; } };

function getMonthName(mo) {
  mo = mo - 1; // Adjust month number for array index (1 = Jan, 12 = Dec)
  var months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul',
                'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  if (months[mo]) {
    return months[mo];
  } else {
    throw 'InvalidMonthNo'; //throw keyword is used here
  }
}

const myMonth = 13; // obvs an error
try { // statements to try
  monthName = getMonthName(myMonth); // function could throw exception
}
catch (e) {
  console.log(e); // pass exception object to error handler -> your own function
}
finally {
	console.log('This always happens after the try catch statements')
}


// Promises

// A Promise is in one of these states:

// pending: initial state, not fulfilled or rejected.
// fulfilled: successful operation
// rejected: failed operation.
// settled: the Promise is either fulfilled or rejected, but not pending.
