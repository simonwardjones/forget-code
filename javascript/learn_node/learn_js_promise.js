	// Promises

	let myFailPromise = () => new Promise((resolve, reject) => {
	  setTimeout(function(){
	    reject("FAIL!"); // Yay! Everything went wrong!
	  }, 250);
	}) // pass up the error

	myFailPromise().catch(e => e).then(e => console.log('then handled:', e))



	let cafeNero = new Promise((res,rej) => {
		rej('this');
	})

	cafeNero.then(() => {
		console.log('all done');
	}).catch((rej) => {
		console.log(rej);
	})


	let time = 17.1;
	var weekend = false;

	var pubSession = new Promise((resolve,reject) => {
		if (weekend) {
			if (check_time()) {
				resolve();
			} else {
				reject('Not time');
			}
		} else {
			reject('Not the weekend');
		}
	})

	function check_time() {
		return time > 17;
	}

	pubSession.then(() => console.log('Pub time!')
		,(err) => console.log(err) )

	function Coffee(type='espresso',volume=50){
		this.type = type;
		this.volume = volume;
		this.sip = function sip(){
			if (this.volume >= 20){
				this.volume -= 20
			} else if (0 < this.volume <= 20){
				this.volume = 0
				console.log("Empty Coffee")
			}
			return this
		}
	}


	function Person(){
		this.drinks = []
	}

	var cap = new Coffee('capacino',200)
	console.log("I just bought",cap)
	console.log("end of program")

