# Ionize

A global helper for cloning git repos into namespaced paths.

Predictable, easy to use, and flexible.

Clone repos via https (default) or ssh :computer:

## Examples

HTTPS:

```bash
$ ionize github.com selfup fut
---> Protocol is: https
---> Ionize is cloning: https://github.com/selfup/fut
---> Into: /Users/RJPB2/Documents/src/github.com/selfup/fut
---> fut has been fetched successfully!
```

SSH

```bash
$ ionize github.com selfup fut ssh
---> Protocol is: ssh
---> Ionize is cloning: git@github.com:selfup/fut
---> Into: /Users/RJPB2/Documents/src/github.com/selfup/fut
---> fut has been fetched successfully!
```

## Install

**From git**: `cargo install --git https://github.com/selfup/ionize.git`

**From path**:

1. Clone repo `git clone https://github.com/selfup/ionize`
2. Have cargo install from local path
3. `cargo install --path <path_to_repo_you_just_cloned>`

No real need to put this up on https://crates.io just yet!

Atleast you get to learn that cargo can install from other sources :smile:

## Paths

Ionize expects an ENV variable called `IONIZED_PATH` to be set.

If it is not set:

1. On macOS/Linux: `$HOME/workspace` will be used.
1. On Windows: `%USERPROFILE&\workspace` will be used.

I like to set mine to `export IONIZED_PATH=$HOME/Documents`

For example you could set: `export IONIZED_PATH=$HOME/Repos`

Now you can use `ionize` to install all repos in `$HOME/Repos/src/domain/author/repo` :tada:

It doesn't have to be rust specific. It's a general helper :smile:

## Usage

0: ionize 1: domain 2: author 3: repo_name 4: ssh

**https**: `ionize github.com selfup ionize`

**ssh**: `ionize github.com selfup ionize ssh`

The domain `github.com` can be:

```js
'gitlab.com' || 'bitbucket.org' || 'my.domain.net';
```
