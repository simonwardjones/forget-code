# learnBash

## General Commands

## Looping

looping on an array example I actually used to start this repo:

```bash
echo "Define an array of languages"
languages=( Python Scala Spark Sublime Bash JavaScript SQL )


echo "Loop over array creating folder and README.md"
for var in ${languages[@]}
do 
	mkdir ${var}
	echo "# learn${var}" > ${var}/Learn"${var}.md"
	echo "making ${var} Folder and README.md"
done

echo "We can access array element using \${languages[0]} = ${languages[0]}" 
echo "We can access array slice using \${languages[@]:0:2} = ${languages[@]:0:2}"
echo "We can print the length of array using \${#languages[@]} = ${#languages[@]}"
echo "We can print the leght of an element e.g. \${#languages[1]} = ${#languages[1]}"
echo "That value should match up with the length of Scala"

```

Genereal looping example on range