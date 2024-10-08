## How can I help

Toml-bombadil is a small project but there is always something to be done, if you wish to contribute there are
several ways you can help us :

### Finding bugs

Found a bug in Toml Bombadil ? Let us know by [opening an issue](https://github.com/oknozor/toml-bombadil/issues/new?assignees=oknozor&labels=bug&template=bug_report.md&title=%5BBUG%5D%5B).

### Writing blogposts

You just migrated your dotfiles to Toml Bombadil and want to share the experience ? Let us know, so we can reference your article.

### Working on existing issues

Toml Bombadil new features are grouped in a milestone for the [next version](https://github.com/oknozor/toml-bombadil/milestones).
You might want to look at this first to get a grasp of want is currently going on.   
If you want to work on an issue, let us know by tagging [@oknozor](https://github.com/oknozor) on the comment section.
We will be happy to provide guidance and respond to your questions.
You might want to pick a tagged [good first issue](https://github.com/oknozor/toml-bombadil/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

### Write bats tests
If you have spotted a bug, a great way to help us improve toml bombadil is to write a
[bats](https://bats-core.readthedocs.io/en/latest/) test to reproduce it, see [HOWTO](https://github.com/oknozor/toml-bombadil/blob/main/bats-tests/HOWTO.md) for more info.

### Suggesting new features

You have an idea for a new feature ? You are welcome to open a [feature request](https://github.com/oknozor/toml-bombadil/issues/new?assignees=oknozor&labels=enhancement&template=feature_request.md&title=%5BFEATURE%5D)
on the issue board.

### Submit a pull request

Toml bombadil respect the following coding standard for any code addition to the main branch :
- We use [clippy](https://github.com/rust-lang/rust-clippy) to spot lints.
- We enforce code formatting with [rustfmt](https://github.com/rust-lang/rustfmt).
- We write test (Code coverage must never decrease when merging to the main branch).
- Commit shall respect the [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) specification

You have picked an issue and started to work on your fork. Time to make a pull request !
If your issue is referenced in the next version milestone, you are expected to submit a pull request to the corresponding
version branch (following semver format and prefixed by "v", ex : "v2.0.0-rc").
If your issue is a bug fix you can submit your PR to main directly and increase the minor version. 


