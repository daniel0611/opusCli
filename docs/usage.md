# Usage
## Add a task
- title is the only required value
- opus replaces @tomorrow and @today with the corresponding dates in 'YYYY-mm-DD' notation
```bash
# add a new task with the following properties:
# title: review and merge pr 5 
# due: @tomorrow 
# tag: #github
# priority: 3
opus add "review and merge pr 5 @tomorrow #github ,,,"
# add a new task with the given title
opus a "review and merge pr 5"
```
## Mark a task as finished
- opus hides finished tasks from the `opus ls` command
```bash
# mark the task with the id 1 as finished 
opus fin 1
# same as above
opus f 1
```
## Delete a task
## List tasks
- opus hides finished tasks from the `opus ls` command
```bash
# list all tasks
opus ls
# list all task with the tag #work
opus ls "#work"
# list all tasks with the priority ,,,
opus ls ",,,"
# list the task with the id 1
opus ls 1
# same as above
opus l 1
```
## Remove all tasks
```bash
# this clears the whole database
opus clear
```

## Export Tasks
- opus exports all tasks to a specified file
```bash
# Exports all tasks in a data.json file
opus export json data
# mycsvfile.csv
opus export csv mycsvfile
# data.tsv
opus export tsv data
```
