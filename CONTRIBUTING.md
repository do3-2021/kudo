# Kubernetes in Rust : The contributing guide.

Thank you for your interest in this project. In order to contribute to this project, please get familiar with the contributing rules and the git workflow.

## Git workflow :

![Git workflow](https://i.imgur.com/WMNLhY8.jpg)

This repository is a fork of ***to be updated***. All changes made to this repository are eventually going to get through a pull request of the upstream repository.

In this repository, you will find 3 types of branches :
- The main branch
- Team's branch
- Features ( or bugfix ) branches

You must code in a feature branch. This feature branch will eventually end up in a PR to your team's branch. At this level, only your team's members are required to review. This PR is annotated with **PR 1** in the image above.

After accepting multiples PRs by your team's members, your team's branch will get to a point where you feel it's ready to close an issue ( more on that later ). At this point, you must open a PR from you team's branch to the main branch ( **PR 2** ). This PR must be reviewed by at least 5 reviewers before getting marked as accepted.

When all the teams are satisfied with the work in the main branch, and everybody agrees on the correctness of what's been done, a new PR, from this fork to the parent repo ( **PR 3** ), will be opened to be discussed with the parent's repo maintainer. 

## How to commit

First, let's list the basic rules :
- A commit message has a title ( summary ), body and footer
- The title should be 50 characters maximum
- Capitalise the first letter of title, don't put a period ( . ) at the end
- Put a blank line between the title, body and footer
- Wrap the commit body at 72 characters
- Try to write the commit body as **bullets** ( optional )
- Use imperative in title ( **Add** instead of **Added**, for example )
- Describe what and why, but not how ( the purpose, not the implementation )
- Footer contains issue number, if applicable ( #78 )

> For more information about the 50/72 characters rule, please visit [this link.](https://dev.to/noelworden/improving-your-commit-message-with-the-50-72-rule-3g79#:~:text=The%2050%2F72%20Rule%20is,pulled%20out%20of%20a%20hat.)

Your commits should follow the [conventional commits specification.](https://www.conventionalcommits.org/en/v1.0.0/#summary)

Please be mindful of this conventionn when writing your commits. CI jobs will check if your commits follow this convention, since these commits might later be used to create changelog using parsers that understand this convention specifically.

## When to make an Issue, and what's its goal ?
An issue should be opened when new functional needs are defined, a new bug is discovered, or a documentation resource is going to be created. You **must** open an issue before opening the corresponding branch.  An issue should not be technical, and should describe what the end user will get. You open an issue when either one of the following condition is true :
- You want to describe what your team is currently working on : this issue is meant to be merge from your team's branch to the main branch
- You want to describe features that you are currently working on : this issue is meant to be merge from a feature branch to your team's branch

## How to make an issue ?

When creating a new issue, you'll be presented with 2 possibles templates. Please choose the one that fits best what you're trying to do.
You can then follow the form.

The title of the issue should contains what it is about in the least amount of words possible ( container health check )
For example : `Container health check`

To describe what you will add to the program, you need to provide a tasks list. For example :
### Tasks list: 
- [x] Create cluster command
- [ ] Delete cluster command

You should tag each tasks with its or their corresponding feature branch.

After creating your issue, if it is a feature issue, you need to create the branch. To the right of the issue in GitHub, you'll find, under `Development`, a button allowing you to `Create a branch`, linked to the issue. You can then click on `create branch`. All work related to this feature issue should be pushed in this branch.

## When to make a PR, and what's its goal ?
There are two kinds of PR you can make : either a feature branch you want to merge into your team's branch, or your team's branch you want to merge into the main branch. You should create the PR as soon as you have created the branch for your issue, though you should mark it as a draft PR. 

## How to make a PR ?

The title of the PR should, if possible, have the same name as its corresponding issue. Anyway, your PR title should be a very short description of what's been done in your PR.
For example : `Container health check`.

You should first give a quick explanation of your works, the technical decisions you are making / you made, and why. Don't hesitate to aerate your explanation.

You should follow the PR template, it will guide you through the essentials parts of your PR.
The PR must have the same task list that the issue, except that each tasks will have sub-tasks, who are technical.
A commit should resolve one sub-task, whenever it's possible.

### Tasks list:
- [x] Create cluster command
    - [x] Command line parsing
    - [x] Understanding what the user wants
    - [x] Calling relevant API endpoints
    - [x] Dealing with errors
- [ ] Delete cluster command
    - [ ] Command line parsing
    - [ ] Understanding what the user wants
    - [x] Calling relevant API endpoints
    - [ ] Dealing with errors

You should link the issue this PR solves : `Close: #15` . Click [here](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) for more information.


## Reviewing PRs 
Team PRs should be verified by all the members of the team. Project PRs should be verified by at least 5 members of the project. Please review code correctness ( in safety, logic and in convention, such as good variable names ). If you think you might have found a bug or something wrong in the code, be precise and clear in your explanation. You should also review the commits history, verifying that all commits are correct, their description clear, the history clean and that the modification made in each commit belongs to the commit they are into.

Please be polite and forgiving. Don't hesitate to ask questions and to make constructive criticism.