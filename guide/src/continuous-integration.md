# Running `moenarchbook` in Continuous Integration

While the following examples use Travis CI, their principles should
straightforwardly transfer to other continuous integration providers as well.

## Ensuring Your Book Builds and Tests Pass

Here is a sample Travis CI `.travis.yml` configuration that ensures `moenarchbook
build` and `moenarchbook test` run successfully. The key to fast CI turnaround times
is caching `moenarchbook` installs, so that you aren't compiling `moenarchbook` on every CI
run.

```yaml
language: rust
sudo: false

cache:
  - cargo

rust:
  - stable

before_script:
  - (test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
  - (test -x $HOME/.cargo/bin/moenarchbook || cargo install --vers "^0.1" moenarchbook)
  - cargo install-update -a

script:
  - moenarchbook build && moenarchbook test # In case of custom book path: moenarchbook build path/to/mybook && moenarchbook test path/to/mybook
```

## Deploying Your Book to GitHub Pages

Following these instructions will result in your book being published to GitHub
pages after a successful CI run on your repository's `master` branch.

First, create a new GitHub "Personal Access Token" with the "public_repo"
permissions (or "repo" for private repositories). Go to your repository's Travis
CI settings page and add an environment variable named `GITHUB_TOKEN` that is
marked secure and *not* shown in the logs.

Whilst still in your repository's settings page, navigate to Options and change the 
Source on GitHub pages to `gh-pages`.

Then, append this snippet to your `.travis.yml` and update the path to the
`book` directory:

```yaml
deploy:
  provider: pages
  skip-cleanup: true
  github-token: $GITHUB_TOKEN
  local-dir: book # In case of custom book path: path/to/mybook/book
  keep-history: false
  on:
    branch: main
```

That's it!

Note: Travis has a new [dplv2](https://blog.travis-ci.com/2019-08-27-deployment-tooling-dpl-v2-preview-release) configuration that is currently in beta. To use this new format, update your `.travis.yml` file to:

```yaml
language: rust
os: linux
dist: xenial

cache:
  - cargo

rust:
  - stable

before_script:
  - (test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
  - (test -x $HOME/.cargo/bin/moenarchbook || cargo install --vers "^0.1" moenarchbook)
  - cargo install-update -a

script:
  - moenarchbook build && moenarchbook test # In case of custom book path: moenarchbook build path/to/mybook && moenarchbook test path/to/mybook
  
deploy:
  provider: pages
  strategy: git
  edge: true
  cleanup: false
  github-token: $GITHUB_TOKEN
  local-dir: book # In case of custom book path: path/to/mybook/book
  keep-history: false
  on:
    branch: main
  target_branch: gh-pages
```

### Deploying to GitHub Pages manually

If your CI doesn't support GitHub pages, or you're deploying somewhere else
with integrations such as Github Pages:
 *note: you may want to use different tmp dirs*:

```console
$> git worktree add /tmp/book gh-pages
$> moenarchbook build
$> rm -rf /tmp/book/* # this won't delete the .git directory
$> cp -rp book/* /tmp/book/
$> cd /tmp/book
$> git add -A
$> git commit 'new book message'
$> git push origin gh-pages
$> cd -
```

Or put this into a Makefile rule:

```makefile
.PHONY: deploy
deploy: book
	@echo "====> deploying to github"
	git worktree add /tmp/book gh-pages
	rm -rf /tmp/book/*
	cp -rp book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages
```

## Deploying Your Book to GitLab Pages
Inside your repository's project root, create a file named `.gitlab-ci.yml` with the following contents:
```yml
stages:
    - deploy

pages:
  stage: deploy
  image: rust
  variables:
    CARGO_HOME: $CI_PROJECT_DIR/cargo
  before_script:
    - export PATH="$PATH:$CARGO_HOME/bin"
    - moenarchbook --version || cargo install moenarchbook
  script:
    - moenarchbook build -d public
  rules:
    - if: '$CI_COMMIT_REF_NAME == "master"'
  artifacts:
    paths:
      - public
  cache:
    paths:
      - $CARGO_HOME/bin
```

After you commit and push this new file, GitLab CI will run and your book will be available!
