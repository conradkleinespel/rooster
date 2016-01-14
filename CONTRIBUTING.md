# Contribution guidelines and tips

Rooster welcomes contribution from everyone. Here are some guidelines and tips
to help you get started.

## Pull requests

Code contributions to Rooster should be made in the form of Github pull requests.

A core contributor — someone with commit access — will review your pull request
do one of two things:

- give feedback that can you can use to improve your pull request,
- merge the changes to the `master` branch.

Keep the first line of the commit message short and to the point. If necessary,
leave a blank line and then describe your changes in more detail. If you are
committing in response to an open issue, add the issue number at the end of the
first line of the commit message.

Here's an example:

```
Fixes users cannot list their apps on OSX #123

On OSX, users were not able to display their list of passwords with "rooster list"
when using the default Terminal.app. Only iTerm worked.
```

## Bug reports

Bug reports must include the following information:

- all the steps you went through to experience the bug, this will help us
reproduce the issue on our side so we can fix it,
- the operating system you are using — including the name of the distribution
if you are using something like Linux or BSD (i.e. Fedora, Ubuntu, OpenBSD),
- what error messages Rooster shows you, if any.

Try to respond to questions from contributors quickly, so that your issue
gets all the attention it needs until it is fixed.
