### **Details::how this works**

you start writing

```cmd
lee open index.txt
```

if it doesn't exists it will create an empty file

directly, it's possible to create a file and insert some text as follows

```cmd
lee new index.txt -f -in
```

* -f: says it's a file
* -in: we are going to insert text, then the editor opens

<br>
<br>

### creating a project

lee defines project as an structure of folder or files

```
lee new myproject -p
``` 
* "myproject" is the name of the project
* -p: says it's a project

projects have default carpets like src, tests

<br>
<br>

### adding a carpet

carpets are individual applications the are part of the main project, they are linked to it and can be referenced by any other file in the project

```
lee add db
```

db is now linked to the current project and could be accessed anywhere
