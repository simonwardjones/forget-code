// import the assert
var assert = require('assert');
//impotr hello for testin
const { testFunction, pi } = require('../hello.js');



// describe is used to define a test sweet and it the test functions
describe('Donkey', function() {
	before(function(){
		console.log("Running this function before the Donkey test sweet")
	})	
  describe('#indexOf()', function() {
    it('should return -1 when the value is not present', function() {
      assert.equal(-1, [1,2,3].indexOf(4));
    });
  });
// The below test will be pending as they have no callback
  describe('hello js tests', function() {
    it('test the testFunction');
    it('test the value of pi is close enough');
  });
});


function add() {
  return Array.prototype.slice.call(arguments).reduce(function(prev, curr) {
    return prev + curr;
  }, 0);
}

describe('add()', function() {
  var tests = [
    {args: [1, 2],       expected: 3},
    {args: [1, 2, 3],    expected: 6},
    {args: [1, 2, 3, 4], expected: 10}
  ];

  tests.forEach(function(test) {
    it('correctly adds ' + test.args.length + ' args', function() {
      var res = add.apply(null, test.args);
      assert.equal(res, test.expected);
    });
  });
});

describe('Simon',function() {
	it("Using setTimeout to simulate asynchronous code!", function(done){
	    setTimeout(function() {
	        console.log("test")
	        done();
	    }, 200);
	});

})