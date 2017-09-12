# learnScala

[back to ForgetCode](../README.md)
<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [About](#about)
- [Scala Set up](#scala-set-up)
    - [Compiling Scala](#compiling-scala)
    - [sbt](#sbt)

<!-- /MarkdownTOC -->

## About 
Scala is a general-purpose programming language providing support for functional programming and object oriented programming with a strong static type system. Scala's design decisions are aimed to address criticisms of Java.

Scala source code is intended to be compiled to Java bytecode, so that the resulting executable code runs on a Java virtual machine. 


## Scala Set up

If Scala is on the path (to check enter scala -version) the REPL can be launched by simpy typing scala.

> “Technically speaking, the scala program is not an interpreter. Behind the scenes, your input is quickly compiled into bytecode, and the bytecode is executed by the Java virtual machine.”

> **Excerpt From: Horstmann, Cay S. “Scala for the Impatient.”**

REPL stands for read evaluate print loop.

### Compiling Scala
In order to run a Scala application you will need to compile the Scala code. This compiled code can be run by scala or it can be packaged into a jar file. In order to do this you will need to use a build tool such as sbt (scala-build-tool), Maven or another tool.


### sbt
The scala build tool is used to compile, run and package Scala code.

In this directory there is a Scala Project called learnScala. I created this by running the following code in the sbt command prompt (To access the sbt command prompt (assuming it is on the PATH) simply enter sbt into the command line)

```sbt
new scala/scala-seed.g8
```
This will prompt you for a name and then creates a  hello world project structure with the given name as below:

```
.
├── build.sbt
├── project
│   ├── Dependencies.scala
│   └── build.properties
└── src
    ├── main
    │   └── scala
    │       └── example
    │           └── Hello.scala
    └── test
        └── scala
            └── example
                └── HelloSpec.scala
```

#### assembly
I have added a plugin to sbt that allows the creation of fat jar files containing all the dependencies. This optionally replaces the package command but will include all the dependencies.

