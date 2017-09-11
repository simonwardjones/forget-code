package simon.scopty
import scopt.OptionParser
import java.io.File


object scoutWords extends Greeting with App {
	// We will create an input parser so we can parse variables to jar file

	case class Config(user: String = "", password: String = "")
	val parser = new scopt.OptionParser[Config]("scoutWords") {
  		head("scoutWords", "1.0")

		opt[String]('u', "user").required().action( (x, c) =>
		    c.copy(user = x) ).text("user is a required string property")

		opt[String]('p', "password").required().valueName("<file>").
		  	action( (x, c) => c.copy(password = x) ).
		    text("password is a required string  property")
		}

	// parser.parse returns Option[Config]
	parser.parse(args, Config()) match {
	  case Some(config) =>
	    // do stuff
	    val user = config.user
	    val password = config.password
	    println("User is " + user)
	    println("Password is " + password)
	  case None =>
	    // arguments are bad, error message will have been displayed
	}

  	println(greeting)
  //	println("user: " + user)
  // println("password: " + password)
	}

trait Greeting {
	lazy val greeting: String = "Hello Carlos"
}
