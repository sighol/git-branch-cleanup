# git-branch-cleanup

![Example image](docs/example.png)

Small cli that prints out the branches that have been merged and offers to delete them.

```shell
cargo install --git https://github.com/sighol/git-branch-cleanup
```

Then you can run these commands in the root directory of your git repository
```
git fetch --prune
git branch-cleanup
```


