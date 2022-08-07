// First a bit of constructors

function Coffee(type='espresso',volume=50,retail_price=2){
	this.type = type;
	this.empty = false;
	this.volume = volume;
	this.retail_price = retail_price;
	this.sip = function sip() {
		if(!this.empty){
			if (this.volume >= 20){
				this.volume -= 20
				console.log('ahh')
			} else {
				this.volume = 0
				this.empty = true
				console.log("Empty Coffee")
			}
		} else {
			console.log("All drunk")
		}
		return this

	}
}

var cap = new Coffee('capacino',200)

function Person(name, mood ='good'){
	this.name = name
	this.drinks = []
	this.money = 10
	this.mood = mood
	this.BuyDrink = (receiver,drink) => BuyDrink(this,receiver,drink)
}

function BuyDrink(buyer,receiver,drink){
	if (buyer.money > drink.retail_price ) {
		buyer.money -= drink.retail_price
		receiver.drinks.push(drink)
	} else {
		throw Error('Not enough Money')
	}
}

// Before we get promising anything

simon = new Person('Simon')
nate  = new Person('Nate')

// simon.BuyDrink(nate,cap)

// while (!nate.drinks[0].empty){
// 	nate.drinks[0].sip()
// }

// Now lets make some promises

askForCoffee = (whoAsked,asked,coffeeRequest) => {
	return new Promise( function(resolve,reject) {
		console.log(whoAsked.name + ': Can I have a '
			+coffeeRequest.type+' please '+asked.name)
		if (asked.mood == 'good') {
			setTimeout(() => {
				try {
					asked.BuyDrink(whoAsked,coffeeRequest)
					resolve()
				} catch(err) {
					console.log("cuaght inside here")
				}
			},4000)
			 //some ponder time to choose a coffee shop
		} else {
			reject("NO COFFEE GO AWAY")
		}
		console.log('Pondering...')
	})
}

simon.money = 10
// simon.mood = 'foul'

askForCoffee(nate,simon,cap)
	.then(() => nate.drinks[0].sip())
	.then(() => console.log('Ahh thanks simon'))
	.then(() => askForCoffee(simon,nate,cap))
	.then(() => simon.drinks[0].sip())
	.catch((e) => console.log(e))


