import Dependencies._

lazy val root = (project in file(".")).
  settings(
    inThisBuild(List(
      organization := "com.example",
      scalaVersion := "2.12.3",
      version      := "0.1.0"
    )),
    name := "learnScala",
    libraryDependencies ++= Seq(
    	"com.github.scopt" % "scopt_2.12" % "3.7.0",
    	scalaTest % Test
    )
  )
