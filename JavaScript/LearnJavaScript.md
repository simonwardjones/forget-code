# learnJavaScript

<sub>[Back to ForgetCode](../README.md)</sub>

<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [General](#general)
- [D3](#d3)
    - [Key Concepts in D3](#key-concepts-in-d3)
        - [Selecting and appending DOM elements](#selecting-and-appending-dom-elements)
        - [Data Joins](#data-joins)
- [Node and NPM](#node-and-npm)
    - [Overview](#overview)
    - [Install and Update Node](#install-and-update-node)

<!-- /MarkdownTOC -->

# General

# D3
Check this link out: [D3 in Depth](http://d3indepth.com/introduction/)

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
```