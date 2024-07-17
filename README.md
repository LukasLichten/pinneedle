# Pinneedle
Dead simple self-hostable blog, using with posts written in markdown.  
It can also use git to update the posts automatically.  

## Running
### Enviroment
| Env              | Description                                          | Default  |
|------------------|------------------------------------------------------|----------|
| `PIN_LOCAL_PATH` | Local folder containing/storing the Blog files       | `./blog` |
| `PIN_BLOG_REPO`  | URL of the remote git repo, cloned into local folder | *unset*  |

If `PIN_BLOG_REPO` is set then the git features are used, and the files in local folder automatically updated

### Dependencies
Requires git (only if `PIN_BLOG_REPO` is set)

## Build instructions
To build use
```
make
```

Requires Rust
