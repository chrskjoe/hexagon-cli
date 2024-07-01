# hexagon-cli
The cli for hexagon, a task and time mangement tool


- [ ] `hexagon -i` interactive mode

## Draft

```
    hexagon create task 
        --name "task name" 
        --project "project id" 
        --topic "topic id1" "topic id2" "topic id3"
        --deadline "2021-12-31" 
        --reminder "2021-12-30"

   
    hexagon create project 
        --name "project name"
        --description "project description" 

    
    hexagon create topic 
        --name "topic name"
        --description "topic description"
        --parent_task "parent_task_name/id"
        --

    hexagon create commit 
        --task "task id" or --project "project id" or --topic "topic id"
        --description "commit message"
        --duratoin "1h 30m" (optional)
        --start-from "2021-12-31 23:59" (optional)

    hexagon set task
        --task "task id" or --project "project id" or --topic "topic id"
        --date "2021-12-31 23:59"
```

## Finsihed 

- hexagon init
- hexagon create 
    - hexagon create task -n "task name"  
        - [ ] hexagon create task -n "task_name" -p "parent_task"
    - hexagon create topic "topic" 
        - hexagon create topic "topic" "parent_id" 

- hexagon list 
    - hexagon list task
    - 
