# Learning R in 10 mins
our_dir <- getwd()
print(our_dir)

# Vectors
##############
example_vec <- c("one","two","three")
print(example_vec[1:2])
class(example_vec)
# alphabetic
sort(example_vec, decreasing=FALSE)

# if statements
##############
age = 18
# if, else and else if works like other languages
if(age >= 18) {
  print("Drive and Vote")
} else if (age >= 16){
  print("Drive")
} else {
  print("Wait")
}

# Strings
##################
str1 = "This is a string"

# String length
nchar(string1)

# You can compare strings where later letters are considered
# greater than
sprintf("Dog > Egg : %s", "Dog" > "Egg")
sprintf("Dog == Egg : %s", "Dog" == "Egg")

# Combine strings and define sperator if any
str2 = paste("Owl", "Bear", sep="")
str2



# Dataframes
#################################
# Create customer data frame
custData = data.frame(name=c("Tom", "Sally", "Sue"),
                      age=c(43, 28, 35))

custData

# Get data in row 1 column 1
custData[1,1]

# Get all data in 1st row
custData[1,1:2]

# Get all ages
custData[1:3, 2]

# Get dimensions
dim(custData)

# Add another record
recordMark = data.frame(name="Mark", age=33)
custData = rbind(custData, recordMark)
custData

# Add a column representing debt
debt = c(0, 25.50, 36, 48.19)
custData = cbind(custData, debt)
custData


# Funtions
#################################
getSum = function(num1, num2){
  return(num1 + num2)
}
