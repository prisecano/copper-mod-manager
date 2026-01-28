# About contributing to Copper Mod Manager
**You can contribute to Copper Mod Manager in several ways.**

---
Copper Mod Manager is open source. Anyone can contribute to the CLI-tool in the public `copper-mod-manager` repository: <https://github.com/github/copper-mod-manager>.

## Philosophy
This project is designed that is:
1. Simple > Performance
2. Only apply performance improvements when **noticably** slower.

## Issues

Issues are used to track tasks that contributors can help with.

If you've found something in the CLI tool, or something in the documentation, that should be updated, search the open issues to see if someone else has reported the same thing. If it's something new, open an issue using a template. We'll use the issue to have a conversation about the problem you'd like to be fixed.

## Pull requests

A pull request is a way to suggest changes in our repository. When we merge those changes, new execution files will be created for release.

We only use Modrinth API to get information of your Minecraft mods and the mods only support for the Fabric Mod loader. We do not accept pull requests that add features that interacts with other third-party tools or integrations unless the majority Minecraft community shifts to a different platform.

### Reviewing your own pull requests

You should always review your own pull request first, before marking it as ready for review by others.

For changes, make sure that you:

- Confirm that the changes meet the philosophy outlined at the Philosophy heading.
- Review the content for technical accuracy.
- Check your changes for grammar, and spelling.
- If there are any failing checks in your pull request, troubleshoot them until they're all passing.

### GitHub-flow / feature branching

1. For a new feature, always create an Issue first to discuss it before working on it.
2. Fork this project.
3. Create relevant branch:
- **Fixes**: `fix/<YOUR-FIX-BRANCH_NAME>`
- **Features**: `feature/<YOUR-FEATURE-BRANCH_NAME>`
4. While starting or at the start of development, create a PR (Pull Request) to this main branch.
5. Discuss and push changes if needed.
6. Done and Lead developer happy? Lead developer will merge your PR into main.
