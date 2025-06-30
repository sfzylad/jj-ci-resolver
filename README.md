# JJ CI RESOLVER

`jj-ci-resolver` is a utility that allows to enrich JJ's commit history with
the status of the CI pipelines for the given revset.

### Limitations

`jj-ci-resolver` currently only works with GitLab.


## Installation
1. Make sure to put the [scripts/jj-helper](./scripts/jj-helper) into the `PATH`
directory.
2. Install the content of this repo with
```shell
cargo install --git https://github.com/sfzylad/jj-ci-resolver.git
```

…or after cloning the repo:
```shell
cargo install --path <path to the cloned repository>
```

3. Add the following template configuration to the JJ config.toml file:
```toml
[templates]
[...]

log_node = '''
coalesce(
  [...]
  if(self.contained_in("present(ci_pending)"), "⌛"),
  if(self.contained_in("present(ci_failures)"), "❌"),
  if(self.contained_in("present(ci_success)"), "✅"),
  [...]
)
'''
```

4. Add the following alias to the config.toml:
```toml
[aliases]
[...]

gitlab = ["util", "exec", "--", "jj-helper"]
```

5. Make a query
Now it should be possible to make a query for a given revset. For example:

```shell
jj gitlab -r 'immutable_heads()' --limit=100
```

### Dependencies
`jj-ci-resolver` requires the following dependencies to be available in the PATH:
- [glab](https://docs.gitlab.com/editor_extensions/gitlab_cli/)
- [jq](https://jqlang.org/)


## How does it work
The `jj-helper` script executes the original revset query and generates a list
of commits that is then injected to the `jj-ci-resolver` binary invocation. The
`jj-ci-resolver` binary then creates a dedicated configuration file for every
repo in `${HOME}/.config/jj/conf.d`. Each file contains `--when.repository`
statement and a set of revset aliases. Each alias for each state:
- ci_pending
- ci_success
- ci_failures

This way the same aliases will return different set of commits for different
repository.

