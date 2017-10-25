# learnBash

<sub>[Back to ForgetCode](../README.md)</sub>
<!-- MarkdownTOC autolink="true" bracket="round" indent="    "-->

- [General Commands](#general-commands)
    - [pwd](#pwd)
    - [cd](#cd)
    - [open](#open)
    - [ls](#ls)
    - [tree](#tree)
    - [cp](#cp)
    - [ln -s](#ln--s)
    - [dirname](#dirname)
    - [basename](#basename)
    - [whoami](#whoami)
    - [chown](#chown)
- [Looping](#looping)

<!-- /MarkdownTOC -->

## General Commands

### pwd

Print current working directory

### cd

Change directory to the specified path

### open

Opens the file using the default application for the file extension.

### ls

List the files in the given path 

|**Options (can be grouped e.g. ls -alt)**| Function|
|----------|---------|
|-l | list with long format|
|-s | list with file size|
|-a | include hidden files|
|-t | show time|
|-i | show inode number|
|-X | sort by extension|


### tree

The tree command creates a file structure diagram. 

The -I 'pattern' argument can be used to exclude files/folders from the tree e.g. `tree -I "venv|__pycache__|target" .`

Example 
```bash
mkdir test
cd test
mkdir dir1 dir2 dir3 dir4
cd dir2
touch file1.log file2.txt fil3.md
cd ../..
tree -I 'dir3' test/
rm -r test/
```
will produce:
```
test/
├── dir1
├── dir2
│   ├── fil3.md
│   ├── file1.log
│   └── file2.txt
└── dir4
```

### cp

The cp command is used to make copies of files and directories

|**Options (can be grouped e.g. ls -alt)**| Function|
|----------|---------|
|-i | prompt before overwrite|
|-r | copy directories recursively|


```bash
echo "Make an example directory which we will make some files in for the demo"
mkdir testFolder && cd testFolder
echo "create a text file example"
touch textFileOne.txt
ls
echo "See that text file and now we will copy it"
cp textFileOne.txt textFileTwo.txt
ls
echo "See it is there"
cd ..
ls
echo "Now copy the whole test"
cp -r testFolder testFolder2
tree -P "test*"
rm -r testFolder testFolder2
echo "Clean Up !"
```


### ln -s 
**Symbolic link Command**

A symbolic link is like a shortcut from one file to another. The contents of a symbolic link are the address of the actual file or folder that is being linked to. The -s argument is required to ensure it is a "soft Link" redirecting to the source.

Each file in your file system is identified by a number called an inode.

A hard link lets you assign a different name to a file in a different location but essentially it is exactly the same file. The key that links the files together is the inode number. Don't use a hard link Simon as the inode number changes on saving but wont update the link.

This is useful to add files to the "usr/local/bin" so that they are on the PATH

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


### dirname
**Directory from full path**

The dirname retrieves the directory path from a full path 

```bash
file_name="this/is/a/long/file/path/with/one/textfile.txt"
echo $file_name
echo $(dirname $file_name)
echo $(basename $file_name)
```

### basename
**File from full path**

The dirname retrieves the directory path from a full path 

```bash
file_name="this/is/a/long/file/path/with/one/textfile.txt"
echo $file_name
echo $(dirname $file_name)
echo $(basename $file_name)
```

### whoami

The whoami command returns the current user

### chown

The chown command changes the owener of a file

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




