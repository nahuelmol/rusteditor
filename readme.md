<div>
  <img src="https://img.shields.io/github/last-commit/nahuelmol/rusteditor"/>
  <img src="https://img.shields.io/github/languages/code-size/nahuelmol/rusteditor"/>
  <img src="https://img.shields.io/github/languages/top/nahuelmol/rusteditor"/>
</div>

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

### creating a WASM project

lee defines project as an structure of folder or files

```
lee new myproject -p
``` 
* "myproject" is the name of the project
* -p: says it's a project

projects have default carpets like src, tests

<br>

### adding a carpet

carpets are individual applications the are part of the main project, they are linked to it and can be referenced by any other file in the project

```
lee add db
```

db is now linked to the current project and could be accessed anywhere


<br>

### express project

```
lee new myproject -exp
```

<br>

### docker support

```
lee new mydocker -docker
```

### deleting project

projects can be deleted by using the following command

```
lee delete app -p
```

"app" is the name of the current project and -p says that it is about a project
