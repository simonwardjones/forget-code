# learnBash
<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [General Commands](#general-commands)
    - [ln -s](#ln--s)
- [Looping](#looping)

<!-- /MarkdownTOC -->

## General Commands

### ln -s 
**Symbolic link Command**

A symbolic link is like a shortcut from one file to another. The contents of a symbolic link are the address of the actual file or folder that is being linked to. The -s argument is required to ensure it is a "soft Link" redirecting to the source.

Each file in your file system is identified by a number called an inode.

A hard link lets you assign a different name to a file in a different location but essentially it is exactly the same file. The key that links the files together is the inode number. Don't use a hard link Simon as the inode number changes on saving but wont update the link.

```bash
echo "Make a test  with two sub directories with a file in each"
mkdir test
cd test/
mkdir Simon Carlos
echo "Carlos Content" > Carlos/Carlos.txt
echo "Simon Content" > Simon/Simon.txt
tree
echo "Create links for each files from the other folder"
echo "One hard link and one soft"
cd Carlos
ln -s ../Simon/Simon.txt simon
cd ../Simon
ln ../Carlos/Carlos.txt Carlos
cd ..
tree
cd Simon
ls -i
cd ../Carlos
ls -i
cd ../..
echo "Note the inode numbers are the same for the hard link"
rm -r test/

```


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