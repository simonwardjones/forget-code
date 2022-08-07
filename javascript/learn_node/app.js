//app.js
// when defining the imports from require the names must match
const { testFunction, pi } = require('./hello.js');
console.log(testFunction);
const out = testFunction();
console.log(out);