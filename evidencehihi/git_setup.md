# What steps did we take to setup the repo?

First we created a new private repo on github. Then we added a standard readme and a permissive license.
Then I created a new cargo (cargo is the package manager for rust) crate(a crate is a package).
That automatically creates an empty git repo. So I added the github repo as remove with: `git remote add origin https://github.com/simon/therepo.git` and checked if it was added properly with `git remote -v`.
Then I added the little rust program (which has a glfw window + a opengl white triangle) and commited everything with `git add --all` because cargo automatically added a .gitignore too.

Then, because I started using VS code with the Rust(rls) plugin. We also needed to share the .vscode folder, for that I copied the .gitignore file contents from github to the current one. 

Okay this document is pretty retarded.