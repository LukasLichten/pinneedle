# Pinneedle
Dead simple self-hostable blog, using with posts written in markdown.  
It can also use git to update the posts automatically.  
A template blog is available here: [pinneedle-blog-template](https://github.com/LukasLichten/pinneedle-template-blog)

## Running
### Docker
This command should do the most:
```
docker run -p 3000:3000 -e PIN_BLOG_REPO='https://github.com/LukasLichten/pinneedle-template-blog.git' pinneedle:latest
```
If you intend (as specified below) you probably want to use some sort of persitent volume.  
Per default the blog is stored at `/app/blog`.  
It is however highly recommended to **NOT** mount the local path folder directly (even if you moved it via `PIN_LOCAL_PATH`),
at least when `PIN_BLOG_REPO` is set, because then it won't clone the repo, and assumes it is already setup.

### Enviroment
| Env              | Description                                          | Default  |
|------------------|------------------------------------------------------|----------|
| `PIN_LOCAL_PATH` | Local folder containing/storing the Blog files       | `./blog` |
| `PIN_BLOG_REPO`  | URL of the remote git repo, cloned into local folder | *unset*  |

If `PIN_BLOG_REPO` is set then the git features are used, and the files in local folder automatically updated

### Dependencies
Requires git (only if `PIN_BLOG_REPO` is set)

## Build instructions
Build Requirements are:
```
rust
libc-dev (or musl-dev on Alpine)
```

To build use
```
make
```


Or Build the docker container
```
make docker-build
```
