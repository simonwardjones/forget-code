# learnJavaScript

<sub>[Back to ForgetCode](../README.md)</sub>

<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [General](#general)
- [MDN Javascript Guide](#mdn-javascript-guide)
    - [Introduction](#introduction)
- [D3](#d3)
    - [Key Concepts in D3](#key-concepts-in-d3)
        - [Selecting and appending DOM elements](#selecting-and-appending-dom-elements)
        - [Data Joins](#data-joins)
- [Node and NPM](#node-and-npm)
    - [Overview](#overview)
    - [Install and Update Node](#install-and-update-node)
    - [Installing npm packages](#installing-npm-packages)
        - [package.json](#packagejson)
    - [Update packages locally](#update-packages-locally)
    - [Remove packages](#remove-packages)
    - [jshint package](#jshint-package)
    - [Mocha package](#mocha-package)

<!-- /MarkdownTOC -->

# General

# MDN Javascript Guide

I am working through the guide (code in learn_javascript.js)
see `https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/` for solid guide

## Introduction


# D3
Check this link out: [D3 in Depth](http:3indepth.com/introduction/)

## Key Concepts in D3


### Selecting and appending DOM elements

`Selections`
- Use d3.select and d3.selectAll to select Dom elements 
- For example to select all elements with class item use d3.selectAll('.item')
- Use .append to add elements
- Use .remove to remove elements
- Use .style and .attr to style and edit selections
- Most selection functions return the selection, meaning that selection functions such as .style, .attr and .on can be chained

`Events`
- Using the .on command we can add event handlers such as "click" and "mouseover"
- In the event callback function the this variable is bound to the DOM element.
   - Note that this is a DOM element and not a D3 selection so if we wish to modify it using D3 we must first select it using d3.select(this).

 `Each and call`
 - .each allows a function to be called on each element of a selection and .call allows a function to be called on the selection itself.

 `Filter and sort`
 - Filter using a function returning true or false
 ```javascript
d3.selectAll('circle')
  .filter(function(d, i) {
    return i % 2 === 0;
  })
  .style('fill', 'orange');
 ```

 ```javascript
   d3.selectAll('.person')
    .sort(function(a, b) {
      return b.score - a.score;
    });
 ```

### Data Joins

`General Update Pattern`

```javascript
 function update(data) {
  var u = d3.select('#content')
    .selectAll('div')
    .data(data);

  u.enter()
    .append('div')
    .merge(u)
    .text(function(d) {
      return d;
    });

  u.exit().remove();
}
```

# Node and NPM

## Overview

When some talks about npm they are either refering to 
  - The website (primary tool for users to discover packages)
  - The registory (database of modules)
  - The npm client (command line tool - how developers publish their packages on the registry )

## Install and Update Node

Node comes with npm but npm is updated often to update run 
```bash
sudo npm install npm@latest -g
npm -v
node -v
```

In order to see the directory where the modules are stored for global modules use the command

```bash
npm config get prefix
```

## Installing npm packages

Packages can be installed locally or globally (to install globally include the -g option)

Say I have a project (an example can be found in the Node directory of the JavaScript section) then running the following install will install the package in `node_modules/` and update (Or create if not already there) the `package.json`

```bash
npm install lodash
```

To install the packages but only as a development dependency use the flag --save-dev e.g. for the testing package mocha:
```bash
npm install mocha --save-dev
```

### package.json

It is better to have a package.json in the module you are coding to record the meta-data inclusing the packages. 

To automatically generate one use the command:
```bash
npm init
```

To automatically add these when installing use the --save flag
> As of npm 5.0.0, installed modules are added as a dependency by default, so the --save option is no longer used.

```bash
npm install lodash --save
```

Running npm install, installs all the dependencies from the package.json
```bash
npm install
```


## Update packages locally

To list the packages in the project use the command:
```bash
npm ls
```

To list outdated packages use:
```bash
npm outdated
```

To update any listed as outdated simply run:
```bash
npm update
```


## Remove packages

To remove packages run:
```bash
npm uninstall --save
```

## jshint package

To install and use:
```bash
npm install -g jshint
jshint app.js
```


## Mocha package

1. `describe()` is merely for grouping, which you can nest 
2. `it()` is a test case
3. `before()`, `beforeEach()`, `after()`, `afterEach()` are hooks to run before/after first/each it() or describe().

To run mocha directly use 
```bash
./node_modules/mocha/bin/mocha
```

It is possible to edit the descibe and it with the .only or .skip extensions to run specific tests.

To use `npm test` one needs to edit the test script command in package json.










